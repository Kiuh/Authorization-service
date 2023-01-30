use crate::{
    rest_api::{authorize::authorize, into_success_response},
    server_state::ServerState,
};
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    login: web::Path<String>,
    request: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let login = login.into_inner();
    authorize(&login, &request, &st.db_connection.pool).await?;
    Response {}.into()
}
