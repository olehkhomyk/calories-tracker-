use crate::config::AppConfig;
use crate::features::auth::jwt::verify_token;
use crate::shared::error::AppError;
use axum::extract::State;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn auth_guard(
    State(config): State<Arc<AppConfig>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let Some(token) = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return Err(AppError::UnAuthorized("Authentication required".into()));
    };

    let Some(claims) = verify_token(&token, config.jwt_secret.as_bytes()).ok()
    else {
        return Err(AppError::UnAuthorized("Invalid Token".into()));
    };

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
