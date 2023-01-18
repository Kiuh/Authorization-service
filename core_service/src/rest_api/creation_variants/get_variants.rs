use crate::database::creation_variants::{FeedType, LifeType, MapName, SetupType, TickPeriod};
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    pub map_names: Vec<String>,
    pub life_types: Vec<String>,
    pub feed_types: Vec<String>,
    pub ticks: Vec<BigDecimal>,
    pub setup_types: Vec<SetupTypeData>,
}
into_success_response!(Response);

#[derive(Serialize, Deserialize)]
pub struct SetupTypeData {
    pub name: String,
    pub json: String,
}

pub async fn execute(st: web::Data<ServerState>) -> actix_web::Result<HttpResponse> {
    Response {
        map_names: MapName::fetch_all(&st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?,
        life_types: LifeType::fetch_all(&st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?,
        feed_types: FeedType::fetch_all(&st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?,
        ticks: TickPeriod::fetch_all(&st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?,
        setup_types: SetupType::fetch_all(&st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?
            .into_iter()
            .map(|set_t| SetupTypeData {
                name: set_t.name,
                json: set_t.json,
            })
            .collect(),
    }
    .into()
}
