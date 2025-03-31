use thiserror::Error;

/// Errors that can occur when using the AccountBuilder.
#[derive(Error, Debug)]
pub enum AccountGenError {
    /// The account owner was not set.
    #[error("Account owner must be set")]
    MissingOwner,

    /// An error occurred during serialization.
    #[error("Failed to serialize data: {0}")]
    SerializationError(#[from] std::io::Error),

    /// An error occurred during deserialization.
    #[error("Failed to deserialize data: {0}")]
    DeserializationError(#[from] borsh::maybestd::io::Error),
} 