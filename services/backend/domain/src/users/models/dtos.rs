//! Internal representation of user model.

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{app_error::AppResult, utils::PasswordManager};
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
    #[validate(length(min = 4, max = 20, message = "Username length must be greater than 4 and less than 20 symbols."))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

impl NewUserDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from<P: PasswordManager>(value: NewUserRequest) -> AppResult<Self> {
        let salt = <P>::generate_salt();
        let hash = <P>::hash_password(&value.password, &salt)?;
        
        Ok(Self {
            name: value.name,
            email: value.email,
            password_hash: hash,
            password_salt: salt,
        })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 4, max = 20, message = "Username length must be greater than 4 and less than 20 symbols."))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
}

impl UpdateUserDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from<P: PasswordManager>(value: UpdateUserRequest) -> AppResult<Self> {
        let (hash, salt) = value.password.map(|pass| {
            let salt = <P>::generate_salt();
            <P>::hash_password(&pass, &salt).map(|hash| (hash, salt))
        }).transpose()?.unzip();

        Ok(Self {
            name: value.name,
            email: value.email,
            password_hash: hash,
            password_salt: salt,
        })
    }
}
