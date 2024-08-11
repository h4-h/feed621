//! Internal app error type.

use std::{borrow::Cow, collections::HashMap};
use validator::ValidationErrors;

/// Alias for errors from serializers and validators.
type ErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

/// Alias for `Result<T, AppError>`.
pub type AppResult<T> = Result<T, AppError>;

/// This is a struct for internal representation of error.
/// It handles all error cases that can happened in the services.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Invaild token
    #[error("Invalid token")]
    BadToken,
    /// Invalid credentials
    #[error("Bad credentials")]
    BadCredentials,
    /// Unauthorized access.
    #[error("Unauthorized access")]
    Unauthorized,
    /// Requested resource not found.
    #[error("{0}")]
    NotFound(String),
    /// Object from request violates some constants.
    #[error("{0}")]
    Constraint(String),
    /// Object from request violates struct scheme.
    #[error("Unprocessible request body")]
    UnprocessibleEntity(ErrorMap),
    #[error("Invalid request body")]
    Validation(#[from] ValidationErrors),
    /// Fallback for any other error.
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}
