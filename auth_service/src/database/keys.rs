use sqlx::{Executor, Postgres};

pub async fn get_public<'a, E>(executor: E) -> crate::error::Result<String>
where
    E: Executor<'a, Database = Postgres>,
{
    Ok(sqlx::query!(r#"SELECT public FROM keys"#)
        .fetch_one(executor)
        .await?
        .public)
}

pub async fn get_private<'a, E>(executor: E) -> crate::error::Result<String>
where
    E: Executor<'a, Database = Postgres>,
{
    Ok(sqlx::query!(r#"SELECT private FROM keys"#)
        .fetch_one(executor)
        .await?
        .private)
}
