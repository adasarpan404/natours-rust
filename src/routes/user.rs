use actix_web::web;

use crate::handlers::user_handler::me;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/me", web::get().to(me)));
}
