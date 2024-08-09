//! Internal representation of topic model.

use serde::{Deserialize, Serialize};
use validator::Validate;
use super::{entities::TopicEntity, schemas::{NewTopicRequest, UpdateTopicRequest}};

#[derive(Serialize, Deserialize)]
pub struct TopicDto {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub query: String,
}

impl From<TopicEntity> for TopicDto {
    fn from(value: TopicEntity) -> Self {
        Self {
            id: value.id,
            owner_id: value.owner_id,
            name: value.name,
            query: value.query,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NewTopicDto {
    pub owner_id: i64,
    #[validate(length(min = 4, max = 20, message = "Topic name length must be greater than 4 and less than 20 symbols."))]
    pub name: String,
    #[validate(length(min = 2, max = 256, message = "Topic query length must be greater than 2 and less than 256 symbols."))]
    pub query: String,
}

impl NewTopicDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from(value: NewTopicRequest, owner_id: i64) -> Self {
        Self {
            owner_id,
            name: value.name,
            query: value.query
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTopicDto {
    pub owner_id: i64,
    #[validate(length(min = 4, max = 20, message = "Topic name length must be greater than 4 and less than 20 symbols."))]
    pub name: Option<String>,
    #[validate(length(min = 2, max = 256, message = "Topic query length must be greater than 2 and less than 256 symbols."))]
    pub query: Option<String>,
}

impl UpdateTopicDto {
    /// `impl From<T>` doesn't accept custom signatures.
    pub fn from(value: UpdateTopicRequest, owner_id: i64) -> Self {
        Self {
            owner_id,
            name: value.name,
            query: value.query
        }
    }
}
