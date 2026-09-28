use axum::Router;

use crate::modules::users;

const BASE_API_URL: &'static str = "/api/v1";

pub fn router() -> Router {
	Router::new()
		.nest(BASE_API_URL, users::router())
}
