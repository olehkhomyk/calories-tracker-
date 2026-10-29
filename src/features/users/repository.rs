use crate::features::users::model::User;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::features::users::dto::CreateUserDto;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create(&self, user: CreateUserDto) -> Result<User, sqlx::Error>;
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

    async fn create(&self, user: CreateUserDto) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password, first_name, last_name, date_of_birth, height, weight)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, email, first_name, last_name, date_of_birth, height, weight, created, updated",
             user.email, user.password, user.first_name, user.last_name, user.date_of_birth, user.height, user.weight
            )
            .fetch_one(&self.pool).await?;

        Ok(result)
    }
}
