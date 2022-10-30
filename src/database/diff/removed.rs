use super::DiffType;
use crate::error::ServerError;
use sqlx::{Executor, Postgres};

pub struct Removed {
    pub local_id: i32,
}

impl Removed {
    pub async fn insert_many<'a, E>(
        events: Vec<Removed>,
        user_login: &str,
        generation_name: &str,
        tick_id: i32,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        let generation_id = sqlx::query!(
            r#"
                SELECT generations.id 
                FROM generations 
                INNER JOIN users 
                ON generations.owner_id = users.id 
                WHERE generations.name = $1 AND users.login = $2
            "#,
            generation_name,
            user_login
        )
        .fetch_one(executor.clone())
        .await
        .map_err(|e| ServerError::Database(e))?
        .id;

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
            DiffType::RemoveCell as DiffType,
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
