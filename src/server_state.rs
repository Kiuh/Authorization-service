use crate::database::DatabaseConnection;

pub struct ServerState {
    pub db_connection: DatabaseConnection,
}

impl ServerState {
    pub async fn new(conn_str: String) -> crate::error::Result<ServerState> {
        Ok(ServerState {
            db_connection: DatabaseConnection::new(&conn_str).await?,
        })
    }
}
