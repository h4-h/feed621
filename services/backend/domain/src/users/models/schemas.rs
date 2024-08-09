//! Request and response structs for user.

use serde::{Deserialize, Serialize};
use crate::users::models::dtos::UserDto;

/// Used in responses for other users.
/// email is ommited for privacy reasons.
#[derive(Serialize, Deserialize)]
pub struct UserPublicResponse {
    pub id: i64,
    pub name: String,
}

impl From<UserDto> for UserPublicResponse {
    fn from(value: UserDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

/// Used in responses when user requests data about themselves.
#[derive(Serialize, Deserialize)]
pub struct UserPrivateResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl From<UserDto> for UserPrivateResponse {
    fn from(value: UserDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
