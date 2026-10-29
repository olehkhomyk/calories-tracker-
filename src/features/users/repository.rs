use crate::features::users::dto::{CreateUserDto, UpdateUserDto};
use crate::features::users::model::User;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;

    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;

    async fn create(&self, user: CreateUserDto) -> Result<User, sqlx::Error>;

    async fn update(&self, id: i64, user: UpdateUserDto) -> Result<Option<User>, sqlx::Error>;

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error>;
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
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, email, first_name, last_name, date_of_birth, height, weight, created, updated
             FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, email, first_name, last_name, date_of_birth, height, weight, created, updated
             FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

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

    async fn update(&self, id: i64, user: UpdateUserDto) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "UPDATE users SET
                email = COALESCE($1, email),
                first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                date_of_birth = COALESCE($4, date_of_birth),
                height = COALESCE($5, height),
                weight = COALESCE($6, weight),
                updated = NOW()
             WHERE id = $7
             RETURNING id, email, first_name, last_name, date_of_birth,
             height, weight, created, updated",
            user.email,
            user.first_name,
            user.last_name,
            user.date_of_birth,
            user.height,
            user.weight,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
