use async_trait::async_trait;
use crate::model::entity::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
}
