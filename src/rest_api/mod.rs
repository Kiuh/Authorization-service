use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod alive;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/User")
            .route("", web::post().to(user::login::execute))
            .route("", web::put().to(user::register::execute)),
    )
    .service(web::scope("/Alive").route("", web::get().to(alive::execute)));

    // cfg.service(
    //     web::scope("/orders")
    //         .route("", web::post().to(order::place_order))
    //         .route("", web::get().to(order::get_orders))
    //         .route("/{id}", web::delete().to(order::cancel_order)),
    // )
    // .service(web::scope("/deposits").route("", web::post().to(deposit::new_deposit)))
    // .service(
    //     web::scope("/markets")
    //         .route("", web::get().to(market::get_markets))
    //         .route("/{id}/candles", web::get().to(market::get_market_candles)),
    // )
    // .service(web::scope("/balances").route("/{user_id}", web::get().to(balance::get_balances)));
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
