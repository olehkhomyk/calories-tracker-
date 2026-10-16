use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
	pub database_pool: PgPool,
}
