use crate::config::AppConfig;
use crate::features::auth::service::AuthService;
use crate::features::users::repository::PostgresUserRepository;
use crate::features::users::service::UserService;
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
	pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool) -> Self {
        let config = Arc::new(config);
		let user_repo = Arc::new(PostgresUserRepository::new(pool));
        let user_service = Arc::new(UserService::new(user_repo));
        let auth_service = Arc::new(
            AuthService::new(
                config.clone(),
                user_service.clone(),
            )
        );

		AppState {
            config,
            user_service,
            auth_service
		}
	}
}

impl FromRef<AppState> for Arc<UserService> {
	fn from_ref(state: &AppState) -> Self {
		state.user_service.clone()
	}
}

impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

