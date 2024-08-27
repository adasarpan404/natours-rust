use actix_web::web;
pub mod auth;

pub fn init(cfg: &mut web::ServiceConfig) {
    auth::init(cfg)
}
