use std::sync::Arc;
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
		let users = self.repository.find_all().await?;

		Ok(users)
	}
}

