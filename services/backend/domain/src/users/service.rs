//! Service, shit that contains logic and validation.

use crate::{app_error::AppResult, utils::Claims};
use super::models::dtos::{NewUserDto, UpdateUserDto, UserDto};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait UserService: Send + Sync {
    async fn create(&self, new_user: NewUserDto) -> AppResult<UserDto>;
    async fn get_by_id(&self, id: i64, claims: Option<Claims>) -> AppResult<UserDto>;
    async fn update(&self, update_user: UpdateUserDto, claims: Option<Claims>) -> AppResult<UserDto>;
    async fn delete(&self, id: i64, claims: Option<Claims>) -> AppResult<()>;
}
