use crate::database::keys;
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    pub pubkey: String,
}
into_success_response!(Response);

pub async fn execute(st: web::Data<ServerState>) -> actix_web::Result<HttpResponse> {
    let pubkey = keys::get_public(&st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response { pubkey }.into()
}
