use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub secret_key: String,
    pub token_expiration: u64,
    pub refresh_token_expiration: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn init() -> Result<Self, ConfigError> {
        dotenv().ok();

        let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
        info!("Loading configuration for environment: {}", env);

        let config = Config::builder()
            // Start with default settings
            .set_default("database.max_connections", 5)?
            .set_default("database.min_connections", 1)?
            .set_default("database.connection_timeout", 30)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8081)?
            .set_default("server.cors_allowed_origins", vec!["http://localhost:3000"])?
            .set_default("logging.level", "info")?
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("APP").separator("_"))
            // Load from environment variables
            .build()?;

        // Deserialize the configuration
        let app_config: AppConfig = config.try_deserialize()?;

        Ok(app_config)
    }

    pub fn get_db_connection_pool_config(&self) -> DBPoolConfig {
        DBPoolConfig {
            url: self.database.url.clone(),
            max_size: self.database.max_connections,
            min_idle: Some(self.database.min_connections),
            connection_timeout: Duration::from_secs(self.database.connection_timeout),
        }
    }
}

pub struct DBPoolConfig {
    pub url: String,
    pub max_size: u32,
    pub min_idle: Option<u32>,
    pub connection_timeout: Duration,
}
