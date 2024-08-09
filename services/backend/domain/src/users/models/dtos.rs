//! Internal representation of user model.

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::utils::PasswordManager;
use super::{schemas::{NewUserRequest, UpdateUserRequest}, entities::UserEntity};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

impl From<UserEntity> for UserDto {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NewUserDto {
    #[validate(custom(function = validate_username_len))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

impl NewUserDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from<P: PasswordManager>(value: NewUserRequest) -> Self {
        let salt = <P>::generate_salt();
        let hash = <P>::hash_password(&value.password, &salt);
        
        Self {
            name: value.name,
            email: value.email,
            password_hash: hash,
            password_salt: salt,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(custom(function = validate_username_len))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
}

impl UpdateUserDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from<P: PasswordManager>(value: UpdateUserRequest) -> Self {
        let (hash, salt) = if value.password.is_some() {
            let salt = <P>::generate_salt();
            let hash = <P>::hash_password(value.password.as_ref().unwrap(), &salt);

            (Some(salt), Some(hash))
        } else {
            (None, None)
        };
        
        Self {
            name: value.name,
            email: value.email,
            password_hash: hash,
            password_salt: salt,
        }
    }
}

// This validation equal to #[validate(length(min = 4, max = 20).
// It done by external function because we have two places where we validate username length.
fn validate_username_len(username: &str) -> Result<(), ValidationError> {
    if username.len() < 4 {
        return Err(ValidationError::new("Username length must be greater than 4 symbols."));
    }
    
    if username.len() > 20 {
        return Err(ValidationError::new("Username length must be less than 20 symbols."))
    }

    Ok(())
}
