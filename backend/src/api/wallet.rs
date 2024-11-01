use actix_web::{get, HttpResponse, Responder};

#[get("/wallet")]
pub async fn wallet_handler() -> impl Responder {
    HttpResponse::Ok().json("Wallet API is working!")
}

