//! CRUD topic repository.

use crate::page::PageEntity;
use super::models::entities::{NewTopicEntity, TopicEntity, UpdateTopicEntity};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait TopicRepository: Send + Sync {
    async fn insert(&self, new_topic: NewTopicEntity) -> anyhow::Result<TopicEntity>;
    async fn select_by_id(&self, topic_id: i64) -> anyhow::Result<TopicEntity>;
    async fn select_many(&self, page: PageEntity) -> anyhow::Result<Vec<TopicEntity>>;
    async fn update(&self, update_topic: UpdateTopicEntity) -> anyhow::Result<TopicEntity>;
    async fn delete(&self, topic_id: i64) -> anyhow::Result<()>;
}
