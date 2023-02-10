use crate::{
    database::{password_recovery, user::User},
    error::{ResponseError, ServerError},
    rest_api::{into_success_response, RsaEncodedParameter},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
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
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let (fetched_nonce, fetched_email) = nonce_email.split_at(nonce_str.len());

    if fetched_nonce != nonce_str {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let user_id = User::get_id(fetched_email, &st.db_connection.pool)
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

    // @TODO: Generate randomly.
    let access_code = "ACCESS_CODE";

    password_recovery::add(user_id, access_code, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    st.mailer
        .send_email(fetched_email, access_code)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {}.into()
}
