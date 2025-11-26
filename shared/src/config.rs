// アプリケーション設定
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub email: EmailConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expires_in_minutes: i64,
    pub refresh_token_expires_in_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub from_address: String,
    pub from_name: String,
    pub verification_url_base: String,
    pub password_reset_url_base: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
