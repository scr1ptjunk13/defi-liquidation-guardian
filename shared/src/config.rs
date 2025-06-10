use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Result}; //Context - removed
use crate::errors::GuardianError;
// you import sibling modules using crate::

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: Option<u32>,
}

impl AppConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| GuardianError::ConfigError(format!(
                "Failed to read config file {}: {}",
                path.display(),
                e
            )))?;

        let mut config: Self = toml::from_str(&content)
            .map_err(|e| GuardianError::ConfigError(format!(
                "Failed to parse TOML from {}: {}",
                path.display(),
                e
            )))?;
        
        //ENV override support
        if let Ok(env_host) = std::env::var("APP_SERVER_HOST") {
            config.server.host = env_host;
        }
        
        if let Ok(env_url) = std::env::var("APP_DATABASE_URL") {
            config.database.url = env_url;
        }
        
        config.validate()?;
        Ok(config)
    }
    
    fn validate(&self) -> Result<(), GuardianError> {
        if self.environment.trim().is_empty() {
            return Err(GuardianError::ConfigError("Environment must not be empty".into()));
        }
        if self.database.url.trim().is_empty() {
            return Err(GuardianError::ConfigError("Database URL must not be empty".into()));
        }
        Ok(())
    }
    
}