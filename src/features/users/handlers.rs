use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use crate::features::users::dto::{CreateUserDto, UpdateUserDto};
use crate::features::users::model::User;
use crate::features::users::service::UserService;

pub async fn list_users(State(user_service): State<Arc<UserService>>) -> Json<Vec<User>> {
	let users = user_service.get_users().await.unwrap(); // Only for now later change handle errors and throw.

	Json(users)
}

pub async fn create_user(
    State(user_service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserDto>,
) -> Json<User> {
	let users = user_service.create_user(payload).await.unwrap(); // Only for now later change handle errors and throw.

	Json(users)
}

pub async fn update_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserDto>,
) -> Json<User> {
    let user = user_service.update_user(id, payload).await.unwrap().unwrap();

    Json(user)
}
