use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;
use serde::Serialize;
use validator::ValidationErrors;

pub enum AppError {
	NotFound(String),
    BadRequest(String),
	UnAuthorized(String),
    Database(sqlx::Error),
    InternalServer(String),
	InvalidJson(JsonRejection),
	Validation(ValidationErrors),
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
impl From<ValidRejection<JsonRejection>> for AppError {
	fn from(error: ValidRejection<JsonRejection>) -> Self {
		match error {
			ValidRejection::Inner(error) => Self::InvalidJson(error),
			ValidRejection::Valid(errors) => Self::Validation(errors)
		}

	}
}

// Lets handlers return `Result<_, AppError>`: Axum calls this to build the HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
	    let mut details: Option<Vec<FieldError>> = None;

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
		    AppError::UnAuthorized(message) => (
			    StatusCode::UNAUTHORIZED,
			    "UNAUTHORIZED",
			    message,
		    ),
		    // TODO check and learn syntax "errors.iter().map(move | ..."
		    AppError::Validation(errors) => {
			    details = Some(
				    errors.field_errors()
					    .into_iter()
					    .flat_map(|(field, errors)| {
						    errors.iter().map(move |error| FieldError {
							    field: field.to_string(),
							    message: error.message
								    .as_deref()
								    .unwrap_or(error.code.as_ref())
								    .to_string(),
						    })
					    })
					    .collect()
			    );

			    (
				    StatusCode::UNPROCESSABLE_ENTITY,
				    "VALIDATION_ERROR",
				    "Validation failed".to_string(),
			    )
		    },
            AppError::InternalServer(error) => {
                eprintln!("Database error: {error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal server error".to_string(),
                )
            },
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
				    error: ErrorBody { code, message, details }
			    }
		    )
	    ).into_response()
    }
}
