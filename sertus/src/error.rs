use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
    #[error("{0}")]
    Error(String),
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("toml error")]
    Toml(#[from] toml::ser::Error),
    #[error("toml serialize error")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml deserialize error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("config error")]
    Config(#[from] sconfig::error::ConfigError),
    #[error("regex error")]
    Regex(#[from] regex::Error),
}

#[macro_export]
macro_rules! app_error {
    ($($arg:tt)*) => {{
        $crate::error::AppError::Error(format!($($arg)*))
    }}
}
