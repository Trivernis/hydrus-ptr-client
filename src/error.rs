use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("api returned error response: {0}")]
    Response(String),

    #[error("failed to parse content as json: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("io error {0}")]
    Io(#[from] std::io::Error),

    #[error("builder error: {0}")]
    Builder(String),

    #[error("malformed response")]
    Malformed,
}
