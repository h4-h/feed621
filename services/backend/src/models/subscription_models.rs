use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "Subscription",
    description = "Full subscription model.",
)]
pub(crate) struct Subscription {
    #[schema(example = 682167858)]
    pub user_id: i64,
    #[schema(example = 1)]
    pub topic_id: i64,
    #[schema(example = 13371337)]
    pub last_post_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "New subscription",
    description = "Subscription model, used in create requests.",
)]
pub(crate) struct NewSubscription {
    #[schema(example = 682167858)]
    pub user_id: i64,
    #[schema(example = 1)]
    pub topic_id: i64,
}
