use crate::{
    database::{password_recovery, user::User},
    error::{ResponseError, ServerError},
    rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub access_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    login: web::Path<String>,
    access_code: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let login = login.into_inner();

    let user_id = User::get_id(&login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?
        .ok_or_else(|| ServerError::UserNotFound(login.clone()).http_status_400())?;

    let stored_access_code = password_recovery::get_access_code(user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    let access_code = access_code.into_inner().access_code;
    if Some(access_code) != stored_access_code {
        return Err(ServerError::WrongAccessCode.http_status_400().into());
    }

    let mut tx = st
        .db_connection
        .pool
        .begin()
        .await
        .map_err(|e| ServerError::Database(e).http_status_500())?;

    password_recovery::apply(user_id, &mut tx)
        .await
        .map_err(|e| e.http_status_500())?;

    tx.commit()
        .await
        .map_err(|e| ServerError::Database(e).http_status_500())?;

    Response {}.into()
}
