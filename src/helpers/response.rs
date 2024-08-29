use serde::Serialize;

use crate::models::auth::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
    pub token: String,
    pub success: bool,
}
