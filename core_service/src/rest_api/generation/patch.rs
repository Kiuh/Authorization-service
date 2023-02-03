use crate::{
    database::generation::GenerationNameDescription,
    error::{ResponseError, ServerError},
    rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    path: web::Path<(i32, String)>, // user_id, name
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let data = data.into_inner();
    let (user_id, name) = path.into_inner();

    let name_descr = GenerationNameDescription {
        name: data.name,
        description: data.description,
    };

    let res = name_descr
        .update(&name, user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if res {
        Response {}.into()
    } else {
        Err(ServerError::UpdateGenerationNameAndDescription(name)
            .http_status_500()
            .into())
    }
}
