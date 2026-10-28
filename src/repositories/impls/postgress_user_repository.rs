use async_trait::async_trait;
use sqlx::PgPool;
use crate::model::entity::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
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
