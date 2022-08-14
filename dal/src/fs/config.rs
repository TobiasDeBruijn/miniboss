use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::fs::FsResult;
use crate::mysql::MysqlConfig;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use cfg_if::cfg_if;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub mysql: MysqlConfig,
    pub fs: FsConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FsConfig {
    pub data_dir: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub allow_registration: bool,
}

impl Config {
    pub async fn read() -> FsResult<Self> {
        let cfg_path = Self::get_config_dir().join("config.toml");
        if !cfg_path.exists() {
            return Self::create_default().await;
        }

        let mut f = fs::File::open(cfg_path).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        let de: Self = toml::de::from_slice(&buf)?;

        Ok(de)
    }

    async fn create_default() -> FsResult<Self> {
        let this = Self::default();
        let ser = toml::ser::to_string_pretty(&this)?;

        fs::create_dir_all(Self::get_config_dir()).await?;

        let mut f = fs::File::create(Self::get_config_dir().join("config.toml")).await?;
        f.write_all(ser.as_bytes()).await?;

        Ok(this)
    }

    fn get_config_dir() -> PathBuf {
        cfg_if! {
            if #[cfg(unix)] {
                PathBuf::from("/etc/miniboss")
            } else if #[cfg(windows)] {
                PathBuf::from_str(r#"C:\Program Data\Miniboss"#)
            } else {
                panic!("Unsupported platform: No configuration directory is defined for this platform.");
            }
        }
    }
}