pub mod dto;
pub mod model;
pub mod repository;
pub mod service;
pub mod handlers;

use axum::Router;
use axum::routing::{get, post, put};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
	Router::new()
		.route("/", get(handlers::list_users))
		.route("/", post(handlers::create_user))
		.route("/{id}", put(handlers::update_user))
}