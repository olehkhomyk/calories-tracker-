use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
	pub id: i64,
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub date_of_birth: Option<NaiveDate>,
	pub height: Option<f64>,
	pub weight: Option<f64>,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime
}
