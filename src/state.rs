use crate::config::AppConfig;
use crate::features::users::repository::PostgresUserRepository;
use crate::features::users::service::UserService;
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
	pub user_service: Arc<UserService>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool) -> Self {
		let user_repo = Arc::new(PostgresUserRepository::new(pool));

		AppState {
            config: Arc::new(config),
			user_service: Arc::new(UserService::new(user_repo)),
		}
	}
}

impl FromRef<AppState> for Arc<UserService> {
	fn from_ref(state: &AppState) -> Self {
		state.user_service.clone()
	}
}


impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

