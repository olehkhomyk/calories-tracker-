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

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub date_of_birth: Option<NaiveDate>
}
