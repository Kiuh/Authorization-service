use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod generation;
pub mod generations;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("User/{user_id}")
            .service(
                web::scope("Generations")
                    .route("", web::get().to(generations::get::execute))
                    .route("", web::post().to(generations::post::execute)),
            )
            .service(
                web::scope("Generation/{name}")
                    .route("", web::delete().to(generation::delete::execute))
                    .route("", web::patch().to(generation::patch::execute))
                    .route("Time", web::get().to(generation::time::get::execute))
                    .service(
                        web::scope("Ticks/{tick}/Cells")
                            .route("", web::get().to(generation::ticks::get::execute))
                            .route("", web::post().to(generation::ticks::post::execute)),
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
