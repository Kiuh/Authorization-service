use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod pubkey;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("User")
            .route("", web::post().to(user::login::execute))
            .route("", web::put().to(user::register::execute))
            .route("", web::patch().to(user::recover_password::execute)),
    )
    .service(web::scope("Pubkey").route("", web::get().to(pubkey::get::execute))) // unstable
    .service(web::scope("Alive").route("", web::get().to(alive::execute)));
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
