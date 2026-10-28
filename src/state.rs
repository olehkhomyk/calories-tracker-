use std::sync::Arc;
use axum::extract::FromRef;
use sqlx::PgPool;
use crate::features::users::repository::PostgresUserRepository;
use crate::features::users::service::UserService;

#[derive(Clone)]
pub struct AppState {
	pub user_service: Arc<UserService>,
}

impl AppState {
	pub fn new(pool: PgPool) -> Self {
		let user_repo = Arc::new(PostgresUserRepository::new(pool));

		AppState {
			user_service: Arc::new(UserService::new(user_repo)),
		}
	}
}

impl FromRef<AppState> for Arc<UserService> {
	fn from_ref(state: &AppState) -> Self {
		state.user_service.clone()
	}
}

