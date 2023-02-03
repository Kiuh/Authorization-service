use crate::{
    database::generation::Generation,
    error::{ResponseError, ServerError},
    rest_api::into_success_response,
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::{FeedTypeJson, LifeTypeJson, MapJson, SetupTypeJson};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub map: MapJson,
    pub feed_type: FeedTypeJson,
    pub setup_type: SetupTypeJson,
    pub life_type: LifeTypeJson,
    pub description: String,
    pub tick: BigDecimal,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
    user_id: web::Path<i32>,
) -> actix_web::Result<HttpResponse> {
    let data = data.into_inner();

    let generation = Generation {
        name: data.name,
        map_prefab: data.map.prefab_name,
        map_json: data.map.json,
        feed_type_prefab: data.feed_type.prefab_name,
        feed_type_json: data.feed_type.json,
        setup_type_prefab: data.setup_type.prefab_name,
        setup_type_json: data.setup_type.json,
        life_type_prefab: data.life_type.prefab_name,
        life_type_json: data.life_type.json,
        description: data.description,
        tick_period: data.tick,
        ..Generation::default()
    };

    let inserted = generation
        .insert(user_id.into_inner(), &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if inserted {
        Response {}.into()
    } else {
        Err(ServerError::InsertGeneration.http_status_500().into())
    }
}
