use anyhow::{anyhow, Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub api_key: String,
    pub port: u16,
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists (ignore if missing)
        dotenvy::dotenv().ok();

        // Load and validate DATABASE_URL
        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is required")?;

        Self::validate_database_url(&database_url)?;

        // Load and validate API_KEY
        let api_key = env::var("API_KEY")
            .context("API_KEY environment variable is required")?;

        if api_key.len() < 32 {
            return Err(anyhow!(
                "API_KEY must be at least 32 characters long (current length: {})",
                api_key.len()
            ));
        }

        // Load optional PORT with default 3000
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        // Load optional RUST_LOG with default "info"
        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            database_url,
            api_key,
            port,
            rust_log,
        })
    }

    /// Validate database URL format
    fn validate_database_url(url: &str) -> Result<()> {
        if !url.starts_with("postgresql://") && !url.starts_with("postgres://") {
            return Err(anyhow!(
                "DATABASE_URL must start with 'postgresql://' or 'postgres://' (got: {})",
                url
            ));
        }
        Ok(())
    }
}
