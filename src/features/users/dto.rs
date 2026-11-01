use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

// Max lengths match the column sizes in migrations/1_create_users.up.sql
#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
	#[validate(email, length(max = 50))]
	pub email: String,
	#[validate(length(min = 8, max = 72))]
	pub password: String,
	#[validate(length(min = 2, max = 30))]
	pub first_name: String,
	#[validate(length(min = 2, max = 30))]
	pub last_name: String,
	#[validate(range(min = 50.0, max = 300.0))]
	pub height: Option<f64>,
	#[validate(range(min = 20.0, max = 500.0))]
	pub weight: Option<f64>,
	pub date_of_birth: Option<NaiveDate>
}

// `Option` fields are validated only when present (`Some`)
#[derive(Deserialize, Validate)]
pub struct UpdateUserDto {
	#[validate(email, length(max = 50))]
	pub email: Option<String>,
	#[validate(length(min = 8, max = 72))]
	pub password: Option<String>,
	#[validate(length(min = 2, max = 30))]
	pub first_name: Option<String>,
	#[validate(length(min = 2, max = 30))]
	pub last_name: Option<String>,
	#[validate(range(min = 50.0, max = 300.0))]
	pub height: Option<f64>,
	#[validate(range(min = 20.0, max = 500.0))]
	pub weight: Option<f64>,
	pub date_of_birth: Option<NaiveDate>
}
