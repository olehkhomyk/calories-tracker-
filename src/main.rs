mod app;
mod config;
mod db;
mod features;
mod state;
mod shared;

use crate::config::AppConfig;
use crate::state::AppState;

#[tokio::main]
async fn main() {
	let config = AppConfig::load()
		.expect("Failed to load application configuration");

	let database_pool = db::create_pool(&config.database_url)
		.await
		.expect("Failed to connect to PostgreSQL");

	sqlx::migrate!("./migrations")
		.run(&database_pool)
		.await
		.expect("Failed to run database migrations");

	let state = AppState::new(database_pool);

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
