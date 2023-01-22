use crate::{
    database::{password_recovery, user::User},
    error::{ResponseError, ServerError},
    rest_api::{decode_rsa_parameter, into_success_response},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub new_password: String, // RSA-OAEP(SHA-256(password), base58), base58, OAEP SHA-256 padding
    pub nonce_email: String,  // SHA-256(nonce + email), base58
    pub nonce: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    login: web::Path<String>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let new_password = decode_rsa_parameter(&data.new_password, "new_password".to_string(), &st)?;
    let login = login.into_inner();

    let user = User::get(&login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?
        .ok_or_else(|| ServerError::UserNotFound(login.clone()).http_status_400())?;

    let mut stored_nonce_email = Sha256::new();
    stored_nonce_email.update(data.nonce.to_string() + &user.data.email);
    let stored_nonce_email = &stored_nonce_email.finalize()[..];
    let stored_nonce_email = bs58::encode(stored_nonce_email).into_string();

    if stored_nonce_email != data.nonce_email {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let nonce_valid =
        password_recovery::try_update_nonce(user.id, data.nonce, &st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?;

    if !nonce_valid {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    // @TODO: Generate randomly.
    let access_code = "ACCESS_CODE";

    password_recovery::add(user.id, &new_password, &access_code, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    let recovery_request = format!("PATCH User/{}/Password?access_code={}", login, access_code);

    st.mailer
        .send_email(&user.data.email, &recovery_request)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {}.into()
}
