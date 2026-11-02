use axum::{
	extract::Request,
	middleware::Next,
	response::Response,
};

pub async fn log_request(req: Request, next: Next) -> Response {
	println!("Before handler");

	println!("{:?} \n", &req.headers());

	let response = next.run(req).await;


	println!("After handler");

	response
}
