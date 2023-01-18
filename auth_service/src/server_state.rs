use crate::database::{keys, DatabaseConnection};
use crate::mail::Mail;

use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    Oaep, PublicKey, RsaPrivateKey, RsaPublicKey,
};

pub struct ServerState {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
    pub db_connection: DatabaseConnection,
    pub mailer: Mail,
}

impl ServerState {
    pub async fn new(
        conn_str: String,
        verification_email: String,
        verification_email_password: String,
    ) -> crate::error::Result<ServerState> {
        let db_connection = DatabaseConnection::new(&conn_str).await?;

        let public_key = keys::get_public(&db_connection.pool).await?;
        let public_key =
            RsaPublicKey::from_pkcs1_pem(&public_key).expect("Wrong public key in database");

        let private_key = keys::get_private(&db_connection.pool).await?;
        let private_key =
            RsaPrivateKey::from_pkcs1_pem(&private_key).expect("Wrong private key in database");

        Ok(ServerState {
            public_key,
            private_key,
            db_connection,
            mailer: Mail::new(verification_email, verification_email_password).await?,
        })
    }
}
