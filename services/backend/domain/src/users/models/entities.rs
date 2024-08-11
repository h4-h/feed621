//! One-to-one representation of database objects.

use serde::{Deserialize, Serialize};
use crate::users::models::dtos::{NewUserDto, UpdateUserDto};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct UserEntity {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserEntity {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

impl From<NewUserDto> for NewUserEntity {
    fn from(value: NewUserDto) -> Self {
        Self {
            name: value.name,
            email: value.email,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserEntity {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
}

impl From<UpdateUserDto> for UpdateUserEntity {
    fn from(value: UpdateUserDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
        }
    }
}
