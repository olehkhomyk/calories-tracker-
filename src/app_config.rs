use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub server_host: String,
	pub server_port: u16,
}

impl AppConfig {
	pub fn load() -> Result<Self, config::ConfigError> {
		dotenvy::dotenv().ok();

		config::Config::builder()
			.add_source(
				config::Environment::with_prefix("APP")
			)
			.build()?
			.try_deserialize()
	}
}
