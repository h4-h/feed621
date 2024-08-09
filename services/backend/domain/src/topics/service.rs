//! Service, shit that contains logic and validation.

use crate::{app_error::AppResult, page::PageDto, utils::Claims};
use super::models::dtos::{NewTopicDto, TopicDto, UpdateTopicDto};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait TopicService: Send + Sync {
    // TODO: get_follower_count(topic_id), find_by_name(topic_name)
    async fn create(&self, new_topic: NewTopicDto, claims: Option<Claims>) -> AppResult<TopicDto>;
    async fn get_by_id(&self, topic_id: i64) -> AppResult<TopicDto>;
    async fn get_many(&self, page: PageDto) -> AppResult<Vec<TopicDto>>;
    async fn update(&self, update_topic: UpdateTopicDto, claims: Option<Claims>) -> AppResult<TopicDto>;
    async fn delete(&self, topic_id: i64, claims: Option<Claims>) -> AppResult<()>;
}
