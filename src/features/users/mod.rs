pub mod dto;
pub mod model;
pub mod repository;
pub mod service;
pub mod handlers;

use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post, put};

pub fn router() -> Router<AppState> {
	Router::new()
		.route("/", get(handlers::get_al_users))
		.route("/{id}", get(handlers::get_user_by_id))
		.route("/", post(handlers::create_user))
		.route("/{id}", put(handlers::update_user))
}
