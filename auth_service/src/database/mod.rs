pub mod private_key;
pub mod user;

use sqlx::PgPool;

use crate::error::ServerError;

pub struct DatabaseConnection {
    pub pool: PgPool,
}

impl DatabaseConnection {
    pub async fn new(conn_str: &str) -> crate::error::Result<DatabaseConnection> {
        Ok(DatabaseConnection {
            pool: PgPool::connect(conn_str)
                .await
                .map_err(|e| ServerError::DatabaseConnection(e.to_string()))?,
        })
    }
}
