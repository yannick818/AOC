use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("IO Error: {0}")]
   Surreal(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AocError>;

