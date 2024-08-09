//! Request and response structs for subscription.

use serde::{Deserialize, Serialize};
use super::dtos::SubscriptionDto;

#[derive(Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
}

impl From<SubscriptionDto> for SubscriptionResponse {
    fn from(value: SubscriptionDto) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            topic_id: value.topic_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewSubscriptionRequest {
    pub user_id: i64,
    pub topic_id: i64,
}
