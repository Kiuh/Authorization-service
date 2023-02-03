use super::DiffType;
use crate::error::ServerError;
use sqlx::{Executor, Postgres};

pub struct Created {
    pub local_id: i32,
}

impl Created {
    pub async fn insert_many<'a, E>(
        events: Vec<Created>,
        generation_id: i32,
        tick_id: i32,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        sqlx::query!(
            r#"
                INSERT INTO 
                diffs(cell_id, tick_id, type) 
                SELECT id, $1, $2 FROM 
                (SELECT * FROM UNNEST($3::INTEGER[]) AS DATA(local_id)) local_ids
                INNER JOIN cells 
                ON local_ids.local_id = cells.local_id 
                WHERE cells.generation_id = $4
            "#,
            tick_id,
            DiffType::CreateCell as DiffType,
            &events
                .into_iter()
                .map(|ev| ev.local_id)
                .collect::<Vec<i32>>(),
            generation_id
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?;

        Ok(())
    }
}
