//! One-to-one representation of database objects.

use serde::{Deserialize, Serialize};
use super::dtos::{NewTopicDto, UpdateTopicDto};

#[derive(Serialize, Deserialize)]
pub struct TopicEntity {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTopicEntity {
    pub owner_id: i64,
    pub name: String,
    pub query: String,
}

impl From<NewTopicDto> for NewTopicEntity {
    fn from(value: NewTopicDto) -> Self {
        Self {
            owner_id: value.owner_id,
            name: value.name,
            query: value.query,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTopicEntity {
    pub name: Option<String>,
    pub query: Option<String>,
}

impl From<UpdateTopicDto> for UpdateTopicEntity {
    fn from(value: UpdateTopicDto) -> Self {
        Self {
            name: value.name,
            query: value.query,
        }
    }
}
