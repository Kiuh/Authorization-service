use crate::database::generation::Generation;
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub time: BigDecimal,
}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    name: web::Path<String>,
    login: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let name = name.into_inner();
    let login = login.into_inner().login;

    let time = Generation::get_time(&name, &login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response { time }.into()
}
