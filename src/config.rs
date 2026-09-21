use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: Option<String>,
    pub auth_token: Option<String>,
    pub local_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = ConfigBuilder::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| AppConfig::load().expect("failed to load config"))
}

pub fn init_config() -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    CONFIG
        .set(config)
        .map_err(|_| anyhow::anyhow!("config already initialized"))
}
