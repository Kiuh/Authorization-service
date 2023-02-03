use crate::{
    database::generation::Generation, error::ResponseError, rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    pub time: BigDecimal,
}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    path: web::Path<(i32, String)>, // user_id, name
) -> actix_web::Result<HttpResponse> {
    let (user_id, name) = path.into_inner();
    let time = Generation::get_time(&name, user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response { time }.into()
}
