use crate::features::users::dto::{CreateUserDto, UpdateUserDto};
use crate::features::users::model::User;
use crate::features::users::repository::UserRepository;
use crate::shared::error::AppError;
use crate::shared::security::hash_password;
use std::sync::Arc;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let records = self.repository.find_all().await?;

        Ok(records)
    }

    pub async fn get_by_user_by_id(&self, user_id: i64) -> Result<User, AppError> {
        let record = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(record)
    }

    pub async fn create_user(&self, mut user_payload: CreateUserDto) -> Result<User, AppError> {
        user_payload.password = hash_password(user_payload.password).await?;

        let record = self.repository.create(user_payload).await?;

        Ok(record)
    }

    pub async fn update_user(
        &self,
        id: i64,
        user_payload: UpdateUserDto,
    ) -> Result<User, AppError> {
        let record = self
            .repository
            .update(id, user_payload)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(record)
    }
}
