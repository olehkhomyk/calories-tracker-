use crate::shared::error::AppError;
use bcrypt::{DEFAULT_COST, hash, verify};

pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|e| AppError::InternalServer(format!("hash task failed: {e}")))?
        .map_err(|e| AppError::InternalServer(format!("bcrypt error: {e}")))
}

pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let password = password.to_owned();
    let hashed_password = hashed_password.to_owned();

    tokio::task::spawn_blocking(move || verify(password, &hashed_password))
        .await
        .map_err(|e| AppError::InternalServer(format!("verify task failed: {e}")))?
        .map_err(|e| AppError::InternalServer(format!("bcrypt error: {e}")))
}
