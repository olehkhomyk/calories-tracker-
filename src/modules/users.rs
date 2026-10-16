pub mod user_handler;
pub mod user_dto;

use axum::{
	routing::get,
	Router,
};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
	Router::new()
		.route("/users", get(user_handler::get_users))
}
