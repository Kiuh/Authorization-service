use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod common_types;
pub mod creation_variants;
pub mod generation;

pub fn config(cfg: &mut web::ServiceConfig) {
    // @TODO: Make module hierarchy fully match request hierarchy
    cfg.service(
        web::scope("User/{user_id}")
            .route("Generations", web::get().to(generation::get::execute))
            .service(
                web::scope("Generation")
                    .route("", web::post().to(generation::create::execute))
                    .route("{name}", web::delete().to(generation::remove::execute))
                    .route("{name}/Time", web::get().to(generation::get_time::execute))
                    .route(
                        "{name}",
                        web::patch().to(generation::change_name_description::execute),
                    )
                    .route(
                        "{name}/Cells/{send_id}",
                        web::get().to(generation::cells::get::execute),
                    )
                    .route(
                        "{name}/Cells/{send_id}",
                        web::patch().to(generation::cells::patch::execute),
                    ),
            ),
    )
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
