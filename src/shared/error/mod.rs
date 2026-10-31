use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub enum AppError {
	NotFound(String),
    BadRequest(String),
    Database(sqlx::Error),
	InvalidJson(JsonRejection)
}

#[derive(Serialize)]
struct ErrorResponse {
	error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
	code: &'static str,
	message: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	details: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
struct FieldError {
	field: String,
	message: String,
}

// Lets `?` turn a `sqlx::Error` into an `AppError` automatically
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::Database(error)
    }
}

// Lets `?` turn a `JsonRejection` into an `AppError` automatically
impl From<JsonRejection> for AppError {
	fn from(error: JsonRejection) -> Self {
		Self::InvalidJson(error)
	}
}

// Lets handlers return `Result<_, AppError>`: Axum calls this to build the HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
	    let (status, code, message) = match self {
		    AppError::NotFound(message) => (
			    StatusCode::NOT_FOUND,
			    "NOT_FOUND",
			    message,
		    ),
		    AppError::BadRequest(message) => (
			    StatusCode::BAD_REQUEST,
			    "BAD_REQUEST",
			    message,
		    ),
		    AppError::InvalidJson(error) => (
			    error.status(),
			    "INVALID_REQUEST_BODY",
			    error.body_text(),
		    ),
		    AppError::Database(error) => {
			    eprintln!("Database error: {error}");

			    (
				    StatusCode::INTERNAL_SERVER_ERROR,
				    "INTERNAL_ERROR",
				    "Internal server error".to_string(),
			    )
		    }
        };

	    (
		    status,
		    Json(
			    ErrorResponse {
				    error: ErrorBody { code, message, details: None }
			    }
		    )
	    ).into_response()
    }
}
