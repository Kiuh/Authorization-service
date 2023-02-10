use crate::{
    database::{password_recovery, user::User},
    error::{IntoHttpResult, ServerError},
    rest_api::{into_success_response, RsaEncodedParameter},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub access_code: String,
    pub new_password: RsaEncodedParameter,
    pub nonce_email: RsaEncodedParameter,
    pub nonce: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let nonce_email = data.nonce_email.decode("nonce_email".to_string(), &st)?;
    let nonce_str = data.nonce.to_string();

    if nonce_str.len() >= nonce_email.len() {
        return Err(ServerError::WrongNonce).into_http_400();
    }

    let (fetched_nonce, fetched_email) = nonce_email.split_at(nonce_str.len());

    if fetched_nonce != nonce_str {
        return Err(ServerError::WrongNonce).into_http_400();
    }

    let user_id = User::get_id(fetched_email, &st.db_connection.pool)
        .await
        .into_http_500()?
        .ok_or_else(|| ServerError::UserNotFound(fetched_email.to_string()))
        .into_http_404()?;

    let nonce_valid =
        password_recovery::try_update_nonce(user_id, data.nonce, &st.db_connection.pool)
            .await
            .into_http_500()?;

    if !nonce_valid {
        return Err(ServerError::WrongNonce).into_http_400();
    }

    let stored_access_code = password_recovery::get_access_code(user_id, &st.db_connection.pool)
        .await
        .into_http_500()?;

    if Some(&data.access_code) != stored_access_code.as_ref() {
        return Err(ServerError::WrongAccessCode).into_http_400();
    }

    let new_password = data.new_password.decode("new_password".to_string(), &st)?;

    let mut tx = st
        .db_connection
        .pool
        .begin()
        .await
        .map_err(ServerError::Database)
        .into_http_500()?;

    password_recovery::apply(user_id, &new_password, &mut tx)
        .await
        .into_http_500()?;

    tx.commit()
        .await
        .map_err(ServerError::Database)
        .into_http_500()?;

    Response {}.into()
}
