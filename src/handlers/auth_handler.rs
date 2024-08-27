use crate::{
    constants::USERS,
    models::{LoginRequest, User},
};
use actix_web::{web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use mongodb::{bson::doc, Database};

async fn hash_user_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

async fn verify_user_password(
    hash: &str,
    password: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&hash)?;

    let is_correct = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_correct)
}

pub async fn signup(db: web::Data<Database>, item: web::Json<User>) -> impl Responder {
    let user_collection = db.collection::<User>(USERS);

    let hashed_password = match hash_user_password(item.password.as_str()).await {
        Ok(password) => password,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let new_user = User::new(item.name.clone(), item.email.clone(), hashed_password);

    let insert_result = user_collection.insert_one(new_user.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(new_user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn login(db: web::Data<Database>, item: web::Json<LoginRequest>) -> impl Responder {
    let user_collection = db.collection::<User>(USERS);

    let filter = doc! {"email": item.email.clone()};

    let existing_user = match user_collection.find_one(filter, None).await {
        Ok(Some(user)) => user, // User found, bind it to `user`
        Ok(None) => return HttpResponse::NotFound().body("User Not Found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let is_correct =
        verify_user_password(existing_user.password.as_str(), item.password.as_str()).await;

    match is_correct {
        Ok(true) => HttpResponse::Ok().json(existing_user),
        Ok(false) => HttpResponse::Unauthorized().body("Incorrect Password"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
