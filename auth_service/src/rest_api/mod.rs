use crate::{
    error::{ResponseError, ServerError},
    server_state::ServerState,
};
use actix_web::web;
use rsa::Oaep;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod authorize;
pub mod proxy;
pub mod pubkey;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("User")
            .route("", web::post().to(user::post::execute))
            .route("{login}", web::post().to(user::login::execute))
            .service(
                web::scope("Password")
                    .route("", web::post().to(user::password::post::execute))
                    .route("", web::patch().to(user::password::patch::execute)),
            ),
    )
    .service(web::scope("Pubkey").route("", web::get().to(pubkey::get::execute)))
    .service(web::scope("Alive").route("", web::get().to(alive::get::execute)))
    .default_service(web::to(proxy::execute));
}

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    success: bool,
    #[serde(flatten)]
    data: T,
}

macro_rules! into_success_response {
    ($ty:ty) => {
        impl From<$ty> for actix_web::Result<HttpResponse> {
            fn from(data: $ty) -> Self {
                Ok(HttpResponse::Ok().json(data))
            }
        }
    };
}
pub(crate) use into_success_response;

// @TODO: implement on type system level.
fn decode_rsa_parameter(
    encoded: &str,
    parameter_name: String,
    st: &web::Data<ServerState>,
) -> actix_web::Result<String> {
    let encoded_bytes = bs58::decode(encoded).into_vec().map_err(|_| {
        ServerError::Base58Decode {
            parameter_name: parameter_name.clone(),
        }
        .http_status_400()
    })?;

    let decoded_bytes = st
        .keys
        .private
        .decrypt(Oaep::new::<sha2::Sha256>(), &encoded_bytes)
        .map_err(|_| {
            ServerError::RsaDecode {
                parameter_name: parameter_name.clone(),
            }
            .http_status_400()
        })?;

    String::from_utf8(decoded_bytes).map_err(|_| {
        ServerError::WrongTextEncoding {
            parameter_name: parameter_name.clone(),
        }
        .http_status_400()
        .into()
    })
}
