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
    pub generations: Vec<GenerationData>,
}
into_success_response!(Response);

#[derive(Serialize, Deserialize)]
struct GenerationData {
    pub name: String,
    pub map: String,
    pub life_type: String,
    pub feed_type: String,
    pub setup_type: String,
    pub tick: BigDecimal,
    pub last_send_num: i64,
    pub setup_json: String,
    pub last_cell_num: i64,
    pub description: String,
}

pub async fn execute(
    st: web::Data<ServerState>,
    login: web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let res_data = Generation::fetch_all(&login.into_inner().login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    Response {
        generations: res_data
            .into_iter()
            .map(|res_data| GenerationData {
                name: res_data.name,
                map: res_data.map_id,
                life_type: res_data.life_type,
                feed_type: res_data.feed_type,
                setup_type: res_data.setup_type,
                tick: res_data.tick_period,
                last_send_num: res_data.last_send_num,
                setup_json: res_data.setup_json,
                last_cell_num: res_data.last_cell_num,
                description: res_data.description,
            })
            .collect(),
    }
    .into()
}
