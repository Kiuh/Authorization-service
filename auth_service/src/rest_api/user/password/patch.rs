use crate::{
    database::{password_recovery, user::User},
    error::{ResponseError, ServerError},
    rest_api::{decode_rsa_parameter, into_success_response},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub access_code: String,
    pub new_password: String, // RSA-OAEP(SHA-256(password), base58), base58, OAEP SHA-256 padding
    pub nonce_email: String,  // RSA-OAEP(nonce + email, base58), base58, OAEP SHA-256 padding
    pub nonce: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let nonce_email = decode_rsa_parameter(&data.nonce_email, "nonce_email".to_string(), &st)?;
    let nonce_str = data.nonce.to_string();

    if nonce_str.len() >= nonce_email.len() {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let (fetched_nonce, fetched_email) = nonce_email.split_at(nonce_str.len());

    if fetched_nonce != nonce_str {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let user_id = User::get_id(&fetched_email, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?
        .ok_or_else(|| ServerError::UserNotFound(fetched_email.to_string()).http_status_404())?;

    let nonce_valid =
        password_recovery::try_update_nonce(user_id, data.nonce, &st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?;

    if !nonce_valid {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let stored_access_code = password_recovery::get_access_code(user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if Some(&data.access_code) != stored_access_code.as_ref() {
        return Err(ServerError::WrongAccessCode.http_status_400().into());
    }

    let new_password = decode_rsa_parameter(&data.new_password, "new_password".to_string(), &st)?;

    let mut tx = st
        .db_connection
        .pool
        .begin()
        .await
        .map_err(|e| ServerError::Database(e).http_status_500())?;

    password_recovery::apply(user_id, &new_password, &mut tx)
        .await
        .map_err(|e| e.http_status_500())?;

    tx.commit()
        .await
        .map_err(|e| ServerError::Database(e).http_status_500())?;

    Response {}.into()
}
