use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
	pub id: u64,
	pub first_name: String,
	pub last_name: String,
}
