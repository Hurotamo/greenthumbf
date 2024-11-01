use actix_web::{get, HttpResponse, Responder};

#[get("/activities")]
pub async fn activities_handler() -> impl Responder {
    HttpResponse::Ok().json("Activities API is working!")
}

