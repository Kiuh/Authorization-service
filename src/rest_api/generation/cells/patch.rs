use super::super::super::common_types::{Cell as JsonCell, Diff};
use crate::database::cell::Cell;
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub added: Vec<JsonCell>,
    pub changes: Vec<Diff>,
    pub deleted: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    _path: web::Path<(String, String)>, // generation_name / sendId
    _login: web::Query<QueryData>,
    _data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    // let data = data.into_inner();
    // let login = login.into_inner().login;
    // let (generation_name, send_id) = path.into_inner();

    Cell::insert_many(vec![], "", "", &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {}.into()

    // let name = name.into_inner();

    // let name_descr = GenerationNameDescription {
    //     name: data.name,
    //     description: data.description,
    // };

    // let res = name_descr
    //     .update(&name, &login.into_inner().login, &st.db_connection.pool)
    //     .await
    //     .map_err(|e| e.http_status_500())?;

    // if res {
    //     Response {}.into()
    // } else {
    //     Err(ServerError::UpdateGenerationNameAndDescription(name)
    //         .http_status_500()
    //         .into())
    // }
}
