use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "Topic",
    description = "Full topic model, some fields are under `skip_serializing`.",
)]
pub(crate) struct Topic {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = 682167858)]
    #[serde(skip_serializing)]
    pub owner_id: i64,
    #[schema(example = "Horse gays")]
    pub name: String,
    #[schema(example = "male/male equine -feral")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "New topic",
    description = "Topic model, used in create requests.",
)]
pub(crate) struct NewTopic {
    #[schema(example = 682167858)]
    pub owner_id: i64,
    #[schema(example = "Horse gays")]
    pub name: String,
    #[schema(example = "male/male equine -feral")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "Update topic",
    description = "Update topic model, used in update requests.",
)]
pub(crate) struct UpdateTopic {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = 682167858)]
    pub owner_id: i64,
    #[schema(example = "Horse gays")]
    pub name: Option<String>,
    #[schema(example = "male/male equine -feral")]
    pub query: Option<String>,
}
