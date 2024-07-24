use serde::{Deserialize, Serialize};

use super::{topic_models::Topic, user_models::User};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct Subscription {
    user: User,
    topic: Topic,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct SubscriptionList(pub Vec<Topic>);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NewSubscription {
    user_id: i64,
    topic_id: i64,
}
