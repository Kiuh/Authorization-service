use actix_web::{web, App, HttpServer};
use clap::Parser;
use std::sync::Arc;

pub mod database;
pub mod error;
pub mod mail;
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
    #[clap(long, env = "VERIFICATION_EMAIL")]
    pub verification_email: String,
    #[clap(long, env = "VERIFICATION_EMAIL_PASSWORD")]
    pub verification_email_password: String,
    #[clap(long, env = "CORE_SERVICE_URI")]
    pub core_service_uri: String,
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();
    let server_state = Arc::new(
        ServerState::new(
            cli.database_url,
            cli.verification_email,
            cli.verification_email_password,
            cli.core_service_uri,
        )
        .await
        .unwrap(),
    );

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
