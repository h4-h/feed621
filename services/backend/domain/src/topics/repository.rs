//! CRUD topic repository.

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait TopicRepository: Send + Sync {
}
