//! CRD subscription repository.

use crate::page::PageEntity;
use super::models::entities::{NewSubscriptionEntity, SubscriptionEntity};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait SubscriptionRepository: Send + Sync {
    async fn insert(&self, new_subscription: NewSubscriptionEntity) -> anyhow::Result<SubscriptionEntity>;
    async fn select_many_by_user_id(&self, user_id: i64, page: PageEntity) -> anyhow::Result<Vec<SubscriptionEntity>>;
    async fn delete(&self, subscription_id: i64) -> anyhow::Result<()>;
}
