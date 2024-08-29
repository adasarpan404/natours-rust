use actix_web::{web, Responder};
use mongodb::Database;

use crate::models::auth::User;

pub async fn me(db: web::Data<Database>, item: web::Json<User>) -> impl Responder {}
