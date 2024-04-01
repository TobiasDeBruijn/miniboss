use color_eyre::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize)]
struct EnvConfig {
    config_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub default_client: DefaultClientConfig,
    pub password_pepper: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub ui_login_path: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct DefaultClientConfig {
    pub redirect_uri: String,
}

impl EnvConfig {
    fn new() -> Result<Self> {
        Ok(envy::from_env()?)
    }
}

impl Config {
    async fn open(path: &Path) -> Result<Self> {
        let mut f = fs::File::open(path).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }
}

pub async fn get_config() -> Result<Config> {
    let env = EnvConfig::new()?;
    let config = Config::open(&env.config_path).await?;
    Ok(config)
}