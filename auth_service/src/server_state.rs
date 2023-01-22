use crate::database::{private_key, DatabaseConnection};
use crate::error::{ResponseError, ServerError};
use crate::mail::Mail;

use actix_web::http::Method;
use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::Client;
use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey, RsaPublicKey};

pub struct ServerKeys {
    pub public: RsaPublicKey,
    pub private: RsaPrivateKey,
}

pub struct CoreService {
    uri: String,
}

impl CoreService {
    pub async fn send_request(
        &self,
        request_method: &Method,
        request_body: actix_web::web::Bytes,
        path: &str,
    ) -> actix_web::Result<HttpResponse> {
        let client = Client::default();
        let uri = format!("http://{}/{}", self.uri, path);

        let mut response = match request_method {
            &Method::GET => client.get(uri),
            &Method::POST => client.post(uri),
            &Method::PUT => client.put(uri),
            &Method::PATCH => client.patch(uri),
            &Method::DELETE => client.delete(uri),
            _ => return Err(ServerError::UnsupportedHttpMethod.http_status_400().into()),
        }
        .append_header((awc::http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .send_body(request_body)
        .await
        .map_err(|_| ServerError::SendRequestToCoreService.http_status_500())?;

        Ok(HttpResponseBuilder::new(response.status()).body(
            response
                .body()
                .await
                .map_err(|_| ServerError::SendRequestToCoreService.http_status_500())?,
        ))
    }
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
            core_service: CoreService {
                uri: core_service_uri,
            },
        })
    }
}
