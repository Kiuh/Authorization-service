use crate::{
    database::user::User,
    error::{ResponseError, ServerError},
    rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use rsa::{
    pkcs8::{EncodePrivateKey, LineEnding},
    Oaep, RsaPrivateKey,
};
use serde::{Deserialize, Serialize};

use std::mem::size_of_val;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub email: String, // RSA-OAEP(nonce + email), base64
    pub nonce: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let nonce_email_data = bs58::decode(&data.email).into_vec().map_err(|_| {
        ServerError::Base58Decode {
            parameter_name: "email".to_string(),
        }
        .http_status_400()
    })?;
    let nonce_email = st
        .private_key
        .decrypt(Oaep::new::<sha2::Sha256>(), &nonce_email_data)
        .map_err(|_| {
            ServerError::RsaDecode {
                parameter_name: "email".to_string(),
            }
            .http_status_400()
        })?;
    let nonce_size = size_of_val(&data.nonce);
    if nonce_email.len() < nonce_size {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }
    let (nonce, email) = nonce_email.split_at(nonce_size);

    let nonce = i64::from_be_bytes(nonce[..].try_into().unwrap());

    if nonce != data.nonce {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let password = User::get_password(&data.email, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?
        .ok_or_else(|| ServerError::UserNotFound(data.email.clone()).http_status_500())?;

    st.mailer
        .send_email(&data.email, &password)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {}.into()
}
