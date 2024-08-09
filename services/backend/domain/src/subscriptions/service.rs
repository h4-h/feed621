//! Service, shit that contains logic and validation.

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait SubscriptionService: Send + Sync {
}
