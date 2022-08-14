use thiserror::Error;

mod config;

pub type FsResult<T> = Result<T, FsError>;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("{0}")]
    TomlDe(#[from] toml::de::Error),
}