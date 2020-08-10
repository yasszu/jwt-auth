use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Validate)]
pub struct FormLogin {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResultToken {
    pub success: bool,
    pub token: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseAccount {
    pub account_id: i32,
    pub email: String,
}