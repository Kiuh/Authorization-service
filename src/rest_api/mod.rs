use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod common_types;
pub mod creation_variants;
pub mod generation;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("User")
            .route("", web::post().to(user::login::execute))
            .route("", web::put().to(user::register::execute)),
    )
    .service(web::scope("/Generations").route("", web::get().to(generation::get::execute)))
    .service(
        web::scope("Generation")
            .route("", web::put().to(generation::create::execute))
            .route("{name}", web::delete().to(generation::remove::execute))
            .route("{name}/Time", web::get().to(generation::get_time::execute))
            .route(
                "{name}",
                web::patch().to(generation::change_name_description::execute),
            )
            .route(
                "{name}/Cells/{sendId}",
                web::get().to(generation::cells::get::execute),
            )
            .route(
                "{name}/Cells/{sendId}",
                web::patch().to(generation::cells::patch::execute),
            ),
    )
    .service(web::scope("Alive").route("", web::get().to(alive::execute)))
    .service(
        web::scope("CreationVariants")
            .route("", web::get().to(creation_variants::get_variants::execute)),
    );
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
