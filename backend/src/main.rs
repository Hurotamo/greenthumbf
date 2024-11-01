use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
mod services;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .configure(api::init_routes)
    })
    .bind(&addr)?
    .run()
    .await
}

