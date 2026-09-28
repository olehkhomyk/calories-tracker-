pub mod user_handler;
pub mod user_dto;

use axum::{
	routing::get,
	Router,
};

pub fn router() -> Router {
	Router::new()
		.route("/users", get(user_handler::get_users))
}
