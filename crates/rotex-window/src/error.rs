use thiserror::Error;

/// Failure reported while creating or driving a window.
#[derive(Debug, Error)]
pub enum Error {
    /// A backend operation failed.
    #[error("Backend error: {0}")]
    Backend(
        /// Backend error message.
        String,
    ),
    /// The window descriptor is not usable.
    #[error("Invalid descriptor: {0}")]
    InvalidDescriptor(
        /// Description of the rejected setting.
        &'static str,
    ),
    /// The requested operation is not available.
    #[error("Unsupported operation: {0}")]
    Unsupported(
        /// Description of the unsupported operation.
        &'static str,
    ),
}
