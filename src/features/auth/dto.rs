use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String,
}
