use crate::error::ServerError;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub mod intellect;
pub mod module;

use intellect::Intellect;
use module::{Module, ModuleWithCellId};

use self::intellect::IntellectWithCellId;

pub struct Cell {
    pub parent_id: u64,
    pub local_id: u64,
    pub modules: Vec<Module>,
    pub intellect: Intellect,
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
}
