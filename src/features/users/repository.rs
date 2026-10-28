use std::sync::Arc;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::features::users::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
}

pub struct PostgresUserRepository {
	pool: PgPool,
}

impl PostgresUserRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
	async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
		let result = sqlx::query_as!(
				User,
				"SELECT id, email, first_name, last_name, date_of_birth, height, weight, created, updated
				 FROM users"
				)
		.fetch_all(&self.pool)
		.await?;

		Ok(result)
	}
}
