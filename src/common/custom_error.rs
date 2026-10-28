use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub enum AppError {
	NotFound(String),
	BadRequest(String),
	Database(sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
	error: String,
}

// Lets `?` turn a `sqlx::Error` into an `AppError` automatically
impl From<sqlx::Error> for AppError {
	fn from(error: sqlx::Error) -> Self {
		AppError::Database(error)
	}
}

// Lets handlers return `Result<_, AppError>`: Axum calls this to build the HTTP response
impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let (status, message) = match self {
			AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
			AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
			AppError::Database(error) => {
				// Log the real cause, but don't leak database details to the client
				eprintln!("Database error: {error}");
				(StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal server error"))
			}
		};

		(status, Json(ErrorResponse { error: message })).into_response()
	}
}
