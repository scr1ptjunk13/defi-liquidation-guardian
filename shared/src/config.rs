use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

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
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let mut config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML config file: {}", path.display()))?;
        
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
    
    fn validate(&self) -> Result<()> {
        if self.environment.trim().is_empty(){
            anyhow::bail!("Environment must not be empty");
        }
        if self.database.url.trim().is_empty(){
            anyhow::bail!("Database URL must not be empty");       
        }
        Ok(())
    }
    
}