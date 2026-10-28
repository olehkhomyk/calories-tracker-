use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::features::users::model::User;
use crate::features::users::service::UserService;

pub async fn list_users(State(service): State<Arc<UserService>>) -> Json<Vec<User>> {
	let users = service.get_users().await.unwrap(); // Only for now later change handle errors and throw.

	Json(users)
}

