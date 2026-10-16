use axum::Router;

use crate::app_state::AppState;
use crate::modules::users;

const BASE_API_URL: &str = "/api/v1";

pub fn router() -> Router<AppState> {
	Router::<AppState>::new()
		.nest(BASE_API_URL, users::router())
}
