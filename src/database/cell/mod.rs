use crate::error::ServerError;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub mod intellect;
pub mod module;

use intellect::Intellect;
use module::{Module, ModuleWithCellId};

use self::intellect::IntellectWithCellId;

pub struct Cell {
    pub parent_id: i64,
    pub local_id: i64,
    pub modules: Vec<Module>,
    pub intellect: Intellect,
}

struct PrefetchedCell {
    pub parent_id: i32,
    pub local_id: i32,
    pub intellect_id: i32,
}

impl Cell {
    pub async fn insert_many<'a, E>(
        cells: Vec<Cell>,
        generation_name: &str,
        user_login: &str,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        let cells_count = cells.len();

        let generation_id = sqlx::query!(
            r#"
                SELECT id 
                FROM generations 
                WHERE name = $1 AND 
                owner_id = (SELECT id FROM users WHERE login = $2)
            "#,
            generation_name,
            user_login
        )
        .fetch_one(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .id;

        // Modules and intellects contain cell_id NOT from database for now.
        let (parent_ids, local_ids, modules, intellects): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = cells
            .into_iter()
            .enumerate()
            .map(|(cell_index, cell)| {
                (
                    cell.parent_id as i32,
                    cell.local_id as i32,
                    cell.modules
                        .into_iter()
                        .map(|module| ModuleWithCellId {
                            module,
                            cell_id: cell_index as i32,
                        })
                        .collect::<Vec<ModuleWithCellId>>(),
                    IntellectWithCellId {
                        intellect: cell.intellect,
                        cell_id: cell_index as i32,
                    },
                )
            })
            .multiunzip();

        let inserted_ids: Vec<i32> = sqlx::query!(
            r#"
                INSERT INTO 
                cells(parent_id, local_id, generation_id) 
                SELECT * FROM UNNEST($1::INTEGER[], $2::INTEGER[], $3::INTEGER[]) 
                RETURNING id
            "#,
            &parent_ids,
            &local_ids,
            &itertools::repeat_n(generation_id, cells_count).collect::<Vec<i32>>()
        )
        .fetch_all(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| res.id)
        .collect();

        // Rewrite cell_ids in modules and intellects.
        let modules: Vec<_> = modules
            .into_iter()
            .flatten()
            .map(|module| ModuleWithCellId {
                module: module.module,
                cell_id: inserted_ids[module.cell_id as usize],
            })
            .collect();
        let intellects: Vec<_> = intellects
            .into_iter()
            .map(|intellect| IntellectWithCellId {
                intellect: intellect.intellect,
                cell_id: inserted_ids[intellect.cell_id as usize],
            })
            .collect();

        ModuleWithCellId::insert_many(modules, executor.clone()).await?;
        IntellectWithCellId::insert_many(intellects, executor).await?;

        Ok(())
    }

    pub async fn fetch_alive<'a, E>(
        generation_name: &str,
        user_login: &str,
        tick_id: i32,
        executor: E,
    ) -> crate::error::Result<Vec<Cell>>
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        let generation_id = sqlx::query!(
            r#"
                SELECT id 
                FROM generations 
                WHERE name = $1 AND 
                owner_id = (SELECT id FROM users WHERE login = $2)
            "#,
            generation_name,
            user_login
        )
        .fetch_one(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .id;

        let alive: Vec<_> = sqlx::query!(
            r#"
                SELECT 
                    local_id, 
                    parent_id, 
                    intellect.id AS intellect_id
                FROM cells 
                INNER JOIN intellect
                ON cells.local_id = intellect.cell_id
                WHERE generation_id = $1 
                AND EXISTS
                (
                    SELECT * FROM diffs WHERE
                    cell_id = cells.id AND
                    tick_id <= $2 AND
                    type = 'create_cell'
                )
                AND NOT EXISTS 
                (
                    SELECT * FROM diffs WHERE 
                    cell_id = cells.id AND 
                    tick_id <= $2 AND 
                    type = 'remove_cell'
                )
            "#,
            generation_id,
            tick_id
        )
        .fetch_all(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| PrefetchedCell {
            local_id: res.local_id,
            parent_id: res.parent_id,
            intellect_id: res.intellect_id,
        })
        .collect();

        let mut cells = vec![];
        for prefetched in alive {
            let modules = sqlx::query!(
                r#"
                    SELECT 
                        modules.name AS module_name,
                        modules.value AS initial_value,
                        (
                            SELECT 
                                new_value 
                            FROM
                                diffs 
                            WHERE 
                                tick_id <= $2 AND 
                                cell_id = $1 AND 
                                changed_module = modules.name
                            ORDER BY tick_id DESC
                            LIMIT 1
                        ) AS changed_value
                    FROM modules
                    WHERE cell_id = $1
                "#,
                prefetched.local_id,
                tick_id
            )
            .fetch_all(executor.clone())
            .await
            .map_err(|e| ServerError::Database(e))?
            .into_iter()
            .map(|res| Module {
                name: res.module_name,
                value: if res.initial_value.is_none() {
                    None
                } else {
                    Some(
                        res.changed_value
                            .unwrap_or_else(|| res.initial_value.unwrap()),
                    )
                },
            })
            .collect();

            cells.push(Cell {
                parent_id: prefetched.parent_id as i64,
                local_id: prefetched.local_id as i64,
                intellect: Intellect::fetch(prefetched.intellect_id, executor.clone()).await?,
                modules,
            });
        }

        Ok(cells)
    }
}
