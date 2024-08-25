use actix_web::{web, HttpResponse, Responder};
use futures_util::stream::TryStreamExt;
use mongodb::Database;

pub async fn create_item(db: web::Data<Database>, item: web::Json<User>) -> impl Responder {}
