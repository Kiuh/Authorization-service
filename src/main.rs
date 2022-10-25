use actix_web::{web, App, HttpServer};
use clap::Parser;
use std::sync::Arc;

pub mod database;
pub mod error;
pub mod rest_api;
pub mod server_state;

use server_state::ServerState;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(long, env = "APP_ENDPOINT")]
    pub app_endpoint: String,
    #[clap(long, env = "DATABASE_URL")]
    pub database_url: String,
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();
    println!("{}", cli.database_url);
    let server_state = Arc::new(ServerState::new(cli.database_url).await.unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&server_state)))
            .configure(rest_api::config)
    })
    .bind(cli.app_endpoint)
    .unwrap()
    .run()
    .await
    .unwrap();
}
