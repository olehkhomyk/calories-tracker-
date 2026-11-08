use crate::shared::error::AppError;
use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};

pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|e| AppError::InternalServer(format!("hash task failed: {e}")))?
        .map_err(|e| AppError::InternalServer(format!("bcrypt error: {e}")))
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    let is_valid = verify(password, hashed_password)?;
    // .map_err(|e| AppError::InternalServer(format!("bcrypt error: {:?}", e)))?;

    Ok(is_valid)
}
