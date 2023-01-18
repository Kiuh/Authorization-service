use crate::database::generation::Generation;
use crate::error::{ResponseError, ServerError};
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    name: web::Path<String>,
    user_id: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let name = name.into_inner();
    let user_id = user_id.into_inner().user_id;

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
