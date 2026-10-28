use std::sync::Arc;
use crate::model::entity::user::User;
use crate::repositories::user_repository::UserRepository;

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

