//! Trait for jwt encapsulation.
//!
//! Responsible for serialization, deserialization and validation.

use serde::{Deserialize, Serialize};
use crate::app_error::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: u64,
}

impl Claims {
    pub fn new(sub: i64, exp_sec: u64) -> Self {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            sub,
            exp: current_time + exp_sec,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        self.exp < current_time
    }

    pub fn sub_eq(&self, id: i64) -> bool {
        self.sub == id
    }
}

/// Encapsulation over jwt realization.
#[cfg_attr(test, mockall::automock)]
pub trait JwtManager {
    fn serialize(payload: Claims) -> AppResult<String>;
    fn deserialize(token: &str) -> AppResult<Claims>;
}

#[cfg(test)]
mod test {
    use super::Claims;

    #[test]
    fn validate_success() {
        let claims = Claims::new(1, 3600);

        assert!(!claims.is_expired());
        assert!(claims.sub_eq(1));
    }

    #[test]
    fn validate_fail() {
        let claims = Claims::new(1, 0);
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(claims.is_expired());
        assert!(!claims.sub_eq(36));
    }
}
