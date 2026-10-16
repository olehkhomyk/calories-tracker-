use axum::extract::State;
use axum::Json;
use crate::app_state::AppState;
use crate::modules::users::user_dto::User;

pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
	println!("{:#?}", state);
	
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
