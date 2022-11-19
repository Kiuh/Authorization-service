use crate::error::ServerError;
use bigdecimal::BigDecimal;
use sqlx::{Executor, Postgres};

pub struct MapName {}

impl MapName {
    pub async fn fetch_all<'a, E>(executor: E) -> crate::error::Result<Vec<String>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT * FROM maps
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| res.name)
        .collect())
    }
}

pub struct LifeType {}

impl LifeType {
    pub async fn fetch_all<'a, E>(executor: E) -> crate::error::Result<Vec<String>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT * FROM life_types
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| res.name)
        .collect())
    }
}

pub struct FeedType {}

impl FeedType {
    pub async fn fetch_all<'a, E>(executor: E) -> crate::error::Result<Vec<String>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT * FROM feed_types
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| res.name)
        .collect())
    }
}

pub struct TickPeriod {}

impl TickPeriod {
    pub async fn fetch_all<'a, E>(executor: E) -> crate::error::Result<Vec<BigDecimal>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT * FROM tick_periods
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| res.period)
        .collect())
    }
}

pub struct SetupType {
    pub name: String,
    pub json: String,
}

impl SetupType {
    pub async fn fetch_all<'a, E>(executor: E) -> crate::error::Result<Vec<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT * FROM setup_types
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| SetupType {
            name: res.name,
            json: res.json,
        })
        .collect())
    }
}
