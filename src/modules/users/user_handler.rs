use axum::Json;
use crate::modules::users::user_dto::User;

pub async fn get_users() -> Json<Vec<User>> {
	let users = vec![
		User {
			id: 1,
			first_name: String::from("Tester1"),
			last_name: String::from("Tester1")
		},
		User {
			id: 2,
			first_name: String::from("Tester2"),
			last_name: String::from("Tester2")
		},
	];

	Json(users)
}
