use crate::features::users::entity::User;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

// Max lengths match the column sizes in migrations/1_create_users.up.sql
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
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
pub struct UpdateUserRequest {
	#[validate(email, length(max = 50))]
	pub email: Option<String>,
	#[validate(length(min = 8, max = 72))]
    // todo auth: remove password from update, reset password should be different functionality
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

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            date_of_birth: user.date_of_birth,
            height: user.height,
            weight: user.weight,
            created: user.created,
            updated: user.updated,
        }
    }
}
