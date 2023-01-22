use crate::error::{ResponseError, ServerError};

use actix_web::http::Method;
use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::Client;

pub struct CoreService {
    uri: String,
}

impl CoreService {
    pub fn new(uri: String) -> CoreService {
        CoreService { uri }
    }

    pub async fn send_request(
        &self,
        method: &Method,
        body: actix_web::web::Bytes,
        path: &str,
    ) -> actix_web::Result<HttpResponse> {
        let client = Client::default();
        let uri = format!("http://{}/{}", self.uri, path);

        let mut response = match method {
            &Method::GET => client.get(uri),
            &Method::POST => client.post(uri),
            &Method::PUT => client.put(uri),
            &Method::PATCH => client.patch(uri),
            &Method::DELETE => client.delete(uri),
            _ => return Err(ServerError::UnsupportedHttpMethod.http_status_400().into()),
        }
        .append_header((awc::http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .send_body(body)
        .await
        .map_err(|_| ServerError::SendRequestToCoreService.http_status_500())?;

        Ok(HttpResponseBuilder::new(response.status()).body(
            response
                .body()
                .await
                .map_err(|_| ServerError::SendRequestToCoreService.http_status_500())?,
        ))
    }
}
