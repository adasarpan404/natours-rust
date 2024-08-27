use actix_web::web;

use crate::handlers::auth_handler::{login, signup};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login)),
    );
}
