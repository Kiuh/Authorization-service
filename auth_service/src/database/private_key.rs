use sqlx::{Executor, Postgres};

pub async fn get<'a, E>(executor: E) -> crate::error::Result<String>
where
    E: Executor<'a, Database = Postgres>,
{
    Ok(sqlx::query!(r#"SELECT key FROM private_key"#)
        .fetch_one(executor)
        .await?
        .key)
}
