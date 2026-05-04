use std::path::{Path as FsPath, PathBuf};

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub verifier_api: VerifierApiSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub dist_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VerifierApiSettings {
    pub base_url: String,
    pub auth_token: String,
}

pub fn load_settings() -> Result<Settings, config::ConfigError> {
    let config_path = std::env::var("APP_CONFIG").unwrap_or_else(|_| {
        if FsPath::new("config.toml").exists() {
            "config.toml".to_string()
        } else {
            "config.example.toml".to_string()
        }
    });

    Config::builder()
        .add_source(config::File::with_name(&config_path))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?
        .try_deserialize()
}
