use std::fmt;
use std::error::Error;
use serde::{Serialize, Deserialize};

/// A custom result type used throughout the application.
/// Wraps around the standard Result type for better readability.
pub type AppResult<T> = Result<T, AppError>;

/// Defines the various error types that can occur in the application.
#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    CustomError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::IoError(ref err) => write!(f, "IO Error: {}", err),
            AppError::JsonError(ref err) => write!(f, "JSON Error: {}", err),
            AppError::CustomError(ref err) => write!(f, "Custom Error: {}", err),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> AppError {
        AppError::JsonError(err)
    }
}

/// Represents the possible states of a document.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DocumentState {
    Unsaved,
    Saved,
    Modified,
}

/// Represents a response to an operation, including success/failure status and a message.
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
}

impl OperationResponse {
    /// Creates a new successful operation response with a custom message.
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
        }
    }

    /// Creates a new failed operation response with a custom message.
    pub fn failure(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
        }
    }
}
