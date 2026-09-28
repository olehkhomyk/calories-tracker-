mod app;
mod modules;
mod app_config;

use app_config::AppConfig;

#[tokio::main]
async fn main() {
	let config = AppConfig::load()
		.expect("Failed to load application configuration");

	let app = app::router();
	let address = format!("{}:{}", config.server_host, config.server_port);

	let listener = tokio::net::TcpListener::bind(&address)
		.await
		.unwrap();

	println!("Server running on http://{address}");

	axum::serve(listener, app)
		.await
		.unwrap();
}
