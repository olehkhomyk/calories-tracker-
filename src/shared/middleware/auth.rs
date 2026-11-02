use crate::shared::error::AppError;
use axum::{
	extract::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};

pub async fn auth_guard(req: Request, next: Next) -> Response {
	let authorized = req
		.headers()
		.get("x-demo-auth")
		.and_then(|value| value.to_str().ok())
		== Some("ok");

	if !authorized {
		return AppError::UnAuthorized("Authentication required".to_string()).into_response();
	}

	next.run(req).await
}
