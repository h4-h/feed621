//! Trait for password managment.
//!
//! Responsible for salt generation and password hashing.

use crate::app_error::AppResult;

/// Hasher and salt generator.
#[cfg_attr(test, mockall::automock)]
pub trait PasswordManager {
    fn generate_salt() -> String;
    fn hash_password(password: &str, salt: &str) -> AppResult<String>;
    fn verify(password_hash: &str, salt: &str) -> AppResult<String>;
}
