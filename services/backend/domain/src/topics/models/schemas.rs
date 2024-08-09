//! Request and response structs for topic.

use serde::{Deserialize, Serialize};
use super::dtos::TopicDto;

#[derive(Serialize, Deserialize)]
pub struct TopicResponse {
    pub id: i64,
    pub name: String,
    pub query: String,
}

impl From<TopicDto> for TopicResponse {
    fn from(value: TopicDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            query: value.query,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTopicRequest {
    pub name: String,
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTopicRequest {
    pub name: Option<String>,
    pub query: Option<String>,
}
