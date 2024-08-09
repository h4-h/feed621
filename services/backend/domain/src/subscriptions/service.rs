//! Service, shit that contains logic and validation.

use crate::{app_error::AppResult, page::PageDto, utils::Claims};
use super::models::dtos::{NewSubscriptionDto, SubscriptionDto};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait SubscriptionService: Send + Sync {
    async fn subscribe(&self, new_subscription: NewSubscriptionDto, claims: Option<Claims>) -> AppResult<SubscriptionDto>;
    async fn get_user_subscriptions(&self, user_id: i64, page: PageDto, claims: Option<Claims>) -> AppResult<SubscriptionDto>;
    async fn unsubscribe(&self, subscription_id: i64, claims: Option<Claims>) -> AppResult<()>;
}
