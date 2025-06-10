use thiserror::Error;
use std::io;

//can implement custom "from" for deeper types if needed later
#[derive(Error, Debug)]
pub enum GuardianError {
    //config-related errors
    #[error("IO error: {0}")]
    ConfigError(String),
    
    //chain interaction errors
    #[error("Chain error: {0}")]
    ChainError(String),
    
    //network / rpc errors
    #[error("Network error: {0}")]
    NetworkError(String),
    
    //environment variable missing
    #[error("Environment variable missing: {0}")]
    MissingEnvVar(String),

    // Wrapping lower-level libraries (using a source)
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("TOML parsing error")]
    Toml(#[from] toml::de::Error),

    #[error("Other: {0}")]
    Other(String),

}
