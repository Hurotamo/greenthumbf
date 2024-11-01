use actix_web::web;

mod wallet;
mod subscription;
mod activities;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(wallet::wallet_handler)
        .service(subscription::subscription_handler)
        .service(activities::activities_handler);
}

