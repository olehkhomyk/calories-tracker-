use crate::features::auth::dto::{AuthResponse, LoginRequest};
use crate::features::auth::service::AuthService;
use crate::features::users::dto::CreateUserRequest;
use crate::shared::error::AppError;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum_valid::{Valid, ValidRejection};
use std::sync::Arc;

pub async fn register(
    State(auth_service): State<Arc<AuthService>>,
    payload: Result<Valid<Json<CreateUserRequest>>, ValidRejection<JsonRejection>>,
) -> Result<Json<AuthResponse>, AppError> {
    let Valid(Json(dto)) = payload?;
    let token = auth_service.register(dto).await?;

    Ok(Json(AuthResponse {
        access_token: token,
    }))
}

pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    payload: Result<Valid<Json<LoginRequest>>, ValidRejection<JsonRejection>>,
) -> Result<Json<AuthResponse>, AppError> {
    let Valid(Json(dto)) = payload?;
    let token = auth_service.login(dto).await?;

    Ok(Json(AuthResponse {
        access_token: token,
    }))
}


