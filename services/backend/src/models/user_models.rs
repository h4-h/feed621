use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "User",
    description = "Full user model, some fields are under `skip_serializing`.",
)]
pub(crate) struct User {
    #[schema(example = 682167858)]
    pub tg_user_id: i64,
    #[serde(skip_serializing)]
    #[schema(example = 665367136)]
    pub tg_chat_id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "New user",
    description = "New user model, used in create requests.",
)]
pub(crate) struct NewUser {
    #[schema(example = 682167858)]
    pub tg_user_id: i64,
    #[schema(example = 665367136)]
    pub tg_chat_id: i64,
}

impl From<NewUser> for User {
    fn from(value: NewUser) -> Self {
        Self {
            tg_user_id: value.tg_user_id,
            tg_chat_id: value.tg_chat_id,
        }
    }
}
