//! One-to-one representation of database objects.

use serde::{Deserialize, Serialize};
use super::dtos::NewSubscriptionDto;

#[derive(Serialize, Deserialize)]
pub struct SubscriptionEntity {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewSubscriptionEntity {
    pub user_id: i64,
    pub topic_id: i64,
}

impl From<NewSubscriptionDto> for NewSubscriptionEntity {
    fn from(value: NewSubscriptionDto) -> Self {
        Self {
            user_id: value.user_id,
            topic_id: value.topic_id,
        }
    }
}
