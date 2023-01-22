use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub pubkey: String, // RSA-OAEP 2048bit, PEM, pkcs1
}
into_success_response!(Response);

pub async fn execute(st: web::Data<ServerState>) -> actix_web::Result<HttpResponse> {
    Response {
        pubkey: st.keys.public.to_pkcs1_pem(LineEnding::CRLF).unwrap(),
    }
    .into()
}
