use crate::{
    database::user::User,
    error::{ResponseError, ServerError},
    rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let user = User {
        login: data.login.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
    };

    let inserted = user
        .try_insert(&st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if inserted {
        Response {}.into()
    } else {
        Err(ServerError::RegisterFailed.http_status_400().into())
    }
}
