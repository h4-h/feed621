//! CRUD user repository.

use crate::{app_error::AppResult, users::models::entities::{NewUserEntity, UpdateUserEntity, UserEntity}};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, new_user: NewUserEntity) -> AppResult<UserEntity>;
    async fn select_by_id(&self, user_id: i64) -> AppResult<UserEntity>;
    async fn update(&self, update_user: UpdateUserEntity) -> AppResult<UserEntity>;
    async fn delete(&self, user_id: i64) -> AppResult<()>;
}
