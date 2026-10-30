use crate::features::users::dto::{CreateUserDto, UpdateUserDto};
use crate::features::users::model::User;
use crate::features::users::service::UserService;
use axum::Json;
use axum::extract::{Path, State};
use std::sync::Arc;

pub async fn get_al_users(State(user_service): State<Arc<UserService>>) -> Json<Vec<User>> {
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
	let user = user_service.update_user(id, payload).await.unwrap().unwrap();  // Only for now later change handle errors and throw.

    Json(user)
}


pub async fn get_user_by_id(
	State(user_service): State<Arc<UserService>>,
	Path(id): Path<i64>,
) -> Json<User> {
	let user = user_service.get_by_user_by_id(id).await.unwrap().unwrap(); // Only for now later change handle errors and throw.

	Json(user)
}
