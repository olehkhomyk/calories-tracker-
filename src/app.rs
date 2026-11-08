use crate::features::auth;
use crate::features::users;
use crate::state::AppState;
use axum::Router;

const BASE_API_URL: &str = "/api/v1";

pub fn router(state: AppState) -> Router<AppState> {
	Router::<AppState>::new()
        .nest(&format!("{BASE_API_URL}/auth"), auth::router())
        .nest(&format!("{BASE_API_URL}/users"), users::router(state))
		// .nest(BASE_API_URL, users::router())
}

