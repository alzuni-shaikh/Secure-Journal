use thiserror::Error;

#[derive(Debug, Error)]
pub enum _AppError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("Input error: {0}")]
    Input(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Unknown error: {0}")]
    Other(String),
}

pub type _Result<T> = std::result::Result<T, _AppError>;
