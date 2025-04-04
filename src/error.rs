use thiserror::Error;

/// Errors that can occur when using the AccountBuilder.
#[derive(Error, Debug)]
pub enum AccountGenError {
    /// The account owner was not set.
    #[error("Account owner must be set")]
    MissingOwner,

    /// The account pubkey was not set.
    #[error("Account pubkey must be set")]
    MissingPubkey,

    /// An error occurred during serialization.
    #[error("Failed to serialize data: {0}")]
    SerializationError(std::io::Error),

    /// An error occurred during deserialization.
    #[error("Failed to deserialize data: {0}")]
    DeserializationError(std::io::Error),
    
    /// A generic IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid account data format.
    #[error("Invalid account data format: {0}")]
    InvalidDataFormat(String),

    /// Insufficient balance for rent exemption.
    #[error("Insufficient balance for rent exemption: required {required} but got {actual}")]
    InsufficientBalance { required: u64, actual: u64 },

    /// Program file not found.
    #[error("Program file not found: {0}")]
    ProgramFileNotFound(String),

    /// Invalid Anchor discriminator.
    #[error("Invalid Anchor discriminator: {0}")]
    InvalidAnchorDiscriminator(String),
} 