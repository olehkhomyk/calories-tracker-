use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub date_of_birth: Option<NaiveDate>
}
