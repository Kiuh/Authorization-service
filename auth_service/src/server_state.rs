use crate::core_service::CoreService;
use crate::database::{private_key, DatabaseConnection};
use crate::mail::Mail;

use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey, RsaPublicKey};

pub struct ServerKeys {
    pub public: RsaPublicKey,
    pub private: RsaPrivateKey,
}

pub struct ServerState {
    pub keys: ServerKeys,
    pub db_connection: DatabaseConnection,
    pub mailer: Mail,
    pub core_service: CoreService,
}

impl ServerState {
    pub async fn new(
        conn_str: String,
        verification_email: String,
        verification_email_password: String,
        core_service_uri: String,
    ) -> crate::error::Result<ServerState> {
        let db_connection = DatabaseConnection::new(&conn_str).await?;

        let private_key = private_key::get(&db_connection.pool).await?;
        let private_key =
            RsaPrivateKey::from_pkcs1_pem(&private_key).expect("Wrong private key in database");
        let public_key = private_key.to_public_key();

        let keys = ServerKeys {
            public: public_key,
            private: private_key,
        };

        Ok(ServerState {
            keys,
            db_connection,
            mailer: Mail::new(verification_email, verification_email_password).await?,
            core_service: CoreService::new(core_service_uri),
        })
    }
}
