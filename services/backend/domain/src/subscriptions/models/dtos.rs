//! Internal representation of subscription model.

use serde::{Deserialize, Serialize};
use super::{entities::SubscriptionEntity, schemas::NewSubscriptionRequest};

#[derive(Serialize, Deserialize)]
pub struct SubscriptionDto {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
}

impl From<SubscriptionEntity> for SubscriptionDto {
    fn from(value: SubscriptionEntity) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            topic_id: value.topic_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewSubscriptionDto {
    pub user_id: i64,
    pub topic_id: i64,
}

impl From<NewSubscriptionRequest> for NewSubscriptionDto {
    fn from(value: NewSubscriptionRequest) -> Self {
        Self {
            user_id: value.user_id,
            topic_id: value.topic_id,
        }
    }
}
