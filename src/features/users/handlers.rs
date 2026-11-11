use crate::features::users::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::features::users::service::UserService;
use crate::shared::error::AppError;
use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum_valid::{Valid, ValidRejection};
use std::sync::Arc;

pub async fn get_al_users(
	State(user_service): State<Arc<UserService>>
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = user_service.get_users().await?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub async fn create_user(
    State(user_service): State<Arc<UserService>>,
    payload: Result<
        Valid<Json<CreateUserRequest>>,
	    ValidRejection<JsonRejection>
    >
) -> Result<Json<UserResponse>, AppError> {
	let Valid(Json(dto)) = payload?;
    let user = user_service.create_user(dto).await?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn update_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<i64>,
    payload: Result<
        Valid<Json<UpdateUserRequest>>,
	    ValidRejection<JsonRejection>
    >
) -> Result<Json<UserResponse>, AppError> {
	let Valid(Json(dto)) = payload?;
	let user = user_service.update_user(id, dto).await?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user_by_id(
	State(user_service): State<Arc<UserService>>,
	Path(id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service.get_user_by_id(id).await?;

    Ok(Json(UserResponse::from(user)))
}
