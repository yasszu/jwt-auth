use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Deserialize, Validate)]
pub struct FormLogin {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}