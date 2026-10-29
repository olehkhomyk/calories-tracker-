use std::sync::Arc;
use crate::features::users::dto::{CreateUserDto, UpdateUserDto};
use crate::features::users::model::User;
use crate::features::users::repository::UserRepository;

pub struct UserService {
	repository:  Arc<dyn UserRepository>,
}

impl UserService {
	pub fn new(repository: Arc<dyn UserRepository>) -> Self {
		Self { repository }
	}

	pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
		let records = self.repository.find_all().await?;

		Ok(records)
	}

    pub async fn create_user(&self, user_payload: CreateUserDto) -> Result<User, sqlx::Error> {
        let record = self.repository.create(user_payload).await?;

        Ok(record)
    }

    pub async fn update_user(&self, id: i64, user_payload: UpdateUserDto) -> Result<Option<User>, sqlx::Error> {
        let record = self.repository.update(id, user_payload).await?;

        Ok(record)
    }
}

