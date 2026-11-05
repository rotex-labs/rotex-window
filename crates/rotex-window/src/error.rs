use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Backend error: {0}")]
    Backend(String),
    #[error("Invalid descriptor: {0}")]
    InvalidDescriptor(&'static str),
    #[error("Unsupported operation: {0}")]
    Unsupported(&'static str),
}
