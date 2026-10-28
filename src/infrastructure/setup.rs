use std::sync::Arc;
use sqlx::PgPool;
use crate::adapters::user::postgress_repository::PostgresUserRepository;
use crate::application::user::user_service::UserService;

pub fn build_user_service(pool: PgPool) -> Arc<UserService> {
	let repository = Arc::new(PostgresUserRepository::new(pool));

	Arc::new(UserService::new(repository))
}

