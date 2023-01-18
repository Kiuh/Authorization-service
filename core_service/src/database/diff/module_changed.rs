use super::DiffType;
use crate::error::ServerError;
use bigdecimal::BigDecimal;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub struct ModuleChanged {
    pub local_id: i32,
    pub module: String,
    pub new_value: BigDecimal,
}

impl ModuleChanged {
    pub async fn insert_many<'a, E>(
        events: Vec<ModuleChanged>,
        user_id: i32,
        generation_name: &str,
        tick_id: i32,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        let generation_id = sqlx::query!(
            r#"
                SELECT id 
                FROM generations 
                WHERE name = $1 AND owner_id = $2
            "#,
            generation_name,
            user_id
        )
        .fetch_one(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .id;

        let (local_ids, module_names, new_values): (Vec<_>, Vec<_>, Vec<_>) = events
            .into_iter()
            .map(|ev| (ev.local_id, ev.module, ev.new_value))
            .multiunzip();

        sqlx::query!(
            r#"
                INSERT INTO 
                diffs(cell_id, tick_id, type, changed_module, new_value) 
                SELECT id, $1, $2, evnts.module, evnts.new_value FROM 
                (SELECT * FROM UNNEST($4::INTEGER[], $5::VARCHAR[], $6::DECIMAL[]) AS DATA(local_id, module, new_value)) evnts
                INNER JOIN cells 
                ON evnts.local_id = cells.local_id 
                WHERE cells.generation_id = $3
            "#,
            tick_id,
            DiffType::CreateCell as DiffType,
            generation_id,
            &local_ids,
            &module_names,
            &new_values
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?;

        Ok(())
    }
}
