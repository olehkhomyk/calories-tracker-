use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email, length(max = 50))]
    pub email: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
}
