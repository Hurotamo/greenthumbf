use actix_web::{get, HttpResponse, Responder};

#[get("/subscription")]
pub async fn subscription_handler() -> impl Responder {
    HttpResponse::Ok().json("Subscription API is working!")
}

