use crate::database::generation::Generation;
use crate::error::{ResponseError, ServerError};
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    path: web::Path<(i32, String)>, // user_id, name
) -> actix_web::Result<HttpResponse> {
    let (user_id, name) = path.into_inner();

    let removed = Generation::remove(&name, user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if removed {
        Response {}.into()
    } else {
        Err(ServerError::GenerationNotFound(name)
            .http_status_400()
            .into())
    }
}
