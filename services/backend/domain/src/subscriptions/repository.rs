//! CRD subscription repository.

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait SubscriptionRepository: Send + Sync {
}
