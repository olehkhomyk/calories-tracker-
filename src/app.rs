use axum::Router;
use crate::features::users;
use crate::state::AppState;

const BASE_API_URL: &str = "/api/v1";

pub fn router() -> Router<AppState> {
	Router::<AppState>::new()
		.nest(&format!("{BASE_API_URL}/users"), users::router())
		// .nest(BASE_API_URL, users::router())
}


