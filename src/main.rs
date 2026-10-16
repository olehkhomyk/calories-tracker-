mod app;
mod modules;
mod app_config;
mod database;
mod app_state;

use app_config::AppConfig;
use crate::app_state::AppState;

#[tokio::main]
async fn main() {
	let config = AppConfig::load()
		.expect("Failed to load application configuration");

	let database_pool = database::create_pool(&config.database_url)
		.await
		.expect("Failed to connect to PostgreSQL");

	let state = AppState {
		database_pool,
	};

	let app = app::router()
		.with_state(state);

	let address = format!("{}:{}", config.server_host, config.server_port);

	let listener = tokio::net::TcpListener::bind(&address)
		.await
		.unwrap();

	println!("PostgreSQL connected");
	println!("Server running on http://{address}");

	axum::serve(listener, app)
		.await
		.unwrap();
}
