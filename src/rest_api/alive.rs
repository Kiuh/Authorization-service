use crate::rest_api::into_success_response;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute() -> actix_web::Result<HttpResponse> {
    Response {}.into()
}
