use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Validate)]
pub struct FormLogin {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String
}