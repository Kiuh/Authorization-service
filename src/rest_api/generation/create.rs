use crate::database::generation::Generation;
use crate::error::{ResponseError, ServerError};
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub map: String,
    pub feed_type: String,
    pub setup_type: String,
    pub life_type: String,
    pub description: String,
    pub tick: BigDecimal,
    pub setup_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
    login: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let data = data.into_inner();

    let generation = Generation {
        name: data.name,
        map_id: data.map,
        feed_type: data.feed_type,
        setup_type: data.setup_type,
        life_type: data.life_type,
        description: data.description,
        tick_period: data.tick,
        setup_json: data.setup_json,
        ..Generation::default()
    };

    let inserted = generation
        .insert(&login.into_inner().login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if inserted {
        Response {}.into()
    } else {
        Err(ServerError::InsertGeneration.http_status_500().into())
    }
}
