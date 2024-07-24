use serde::{Deserialize, Serialize};
use super::user_models::User;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct Topic {
    id: i64,
    owner: User,
    label: String,
    query: String,
    sub_counter: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NewTopic {
    owner: User,
    label: String,
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdateTopic {
    label: Option<String>,
    query: Option<String>,
}
