use crate::database::user::User;
use crate::error::{ResponseError, ServerError};
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let exists = User::is_exists(&data.login, &data.password, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;
    if exists {
        Response {}.into()
    } else {
        Err(ServerError::LogInFailed.http_status_500().into())
    }
}
