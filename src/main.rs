use actix_web::{web, App, HttpServer};
use routes::init;

mod constants;
mod db;
mod env;
mod handlers;
mod helpers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::get_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(init)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
