
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {

    #[error("AocError: {0}")]
    Generic(String),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Parse Error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Request Error: {0}")]
    Request(#[from] reqwest::Error),
}

impl std::convert::From<&str> for AocError {
    fn from(value: &str) -> Self {
        Self::Generic(value.to_owned())
    }
}

pub type Result<T> = std::result::Result<T, AocError>;

