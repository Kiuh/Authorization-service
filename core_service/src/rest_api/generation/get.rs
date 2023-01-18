use crate::database::generation::Generation;
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::{FeedTypeJson, LifeTypeJson, MapJson, SetupTypeJson};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub generations: Vec<GenerationJson>,
}
into_success_response!(Response);

#[derive(Serialize, Deserialize)]
struct GenerationJson {
    pub name: String,

    pub map: MapJson,
    pub life_type: LifeTypeJson,
    pub feed_type: FeedTypeJson,
    pub setup_type: SetupTypeJson,

    pub tick: BigDecimal,
    pub last_send_num: i64,
    pub last_cell_num: i64,
    pub description: String,
}

pub async fn execute(
    st: web::Data<ServerState>,
    user_id: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let user_id = user_id.into_inner().user_id;
    let res_data = Generation::fetch_all(user_id, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {
        generations: res_data
            .into_iter()
            .map(|res_data| GenerationJson {
                name: res_data.name,
                map: MapJson {
                    prefab_name: res_data.map_prefab,
                    json: res_data.map_json,
                },
                life_type: LifeTypeJson {
                    prefab_name: res_data.life_type_prefab,
                    json: res_data.life_type_json,
                },
                feed_type: FeedTypeJson {
                    prefab_name: res_data.feed_type_prefab,
                    json: res_data.feed_type_json,
                },
                setup_type: SetupTypeJson {
                    prefab_name: res_data.setup_type_prefab,
                    json: res_data.setup_type_json,
                },
                tick: res_data.tick_period,
                last_send_num: res_data.last_send_num,
                last_cell_num: res_data.last_cell_num,
                description: res_data.description,
            })
            .collect(),
    }
    .into()
}
