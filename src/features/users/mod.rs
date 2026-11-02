pub mod dto;
pub mod model;
pub mod repository;
pub mod service;
pub mod handlers;

use crate::shared::middleware::auth::auth_guard;
use crate::state::AppState;
use axum::routing::{get, post, put};
use axum::{Router, middleware};

pub fn router() -> Router<AppState> {
	Router::new()
		.route("/", get(handlers::get_al_users))
		.route("/{id}", get(handlers::get_user_by_id))
		.route("/", post(handlers::create_user))
		.route("/{id}", put(handlers::update_user))
		.layer(middleware::from_fn(auth_guard))
}
