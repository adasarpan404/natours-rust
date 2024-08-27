use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::env::JWT_SECRET;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: usize,
}

pub fn create_jwt(id: String) -> Result<String, Box<dyn Error>> {
    let my_claims = Claims {
        id,
        exp: (SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600) as usize,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?;

    Ok(token)
}
