use actix_web::web;
pub mod auth;
pub mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    auth::init(cfg)
}
