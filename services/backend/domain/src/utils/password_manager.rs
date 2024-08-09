//! Trait for password managment.
//!
//! Responsible for salt generation and password hashing.

/// Hasher and salt generator.
#[cfg_attr(test, mockall::automock)]
pub trait PasswordManager {
    fn generate_salt() -> String;
    fn hash_password(password: &str, salt: &str) -> String;
}
