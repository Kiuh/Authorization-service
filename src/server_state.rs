use crate::database::DatabaseConnection;
use crate::mail::Mail;

pub struct ServerState {
    pub db_connection: DatabaseConnection,
    pub mailer: Mail,
}

impl ServerState {
    pub async fn new(
        conn_str: String,
        verification_email: String,
        verification_email_password: String,
    ) -> crate::error::Result<ServerState> {
        Ok(ServerState {
            db_connection: DatabaseConnection::new(&conn_str).await?,
            mailer: Mail::new(verification_email, verification_email_password).await?,
        })
    }
}
