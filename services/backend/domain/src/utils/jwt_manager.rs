//! Trait for jwt encapsulation.
//!
//! Responsible for serialization, deserialization and validation.

use serde::{Deserialize, Serialize};

use crate::app_error::AppResult;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: i64,
    exp: u64,
}

/// Encapsulation over jwt realization.
#[cfg_attr(test, mockall::automock)]
pub trait JwtManager {
    fn serialize(payload: Claims) -> AppResult<String>;
    fn deserialize(token: &str) -> AppResult<Claims>;
    fn validate(token: &str, key: &str) -> bool;
}
