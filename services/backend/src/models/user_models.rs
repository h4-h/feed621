use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "User",
    description = "Full user model.",
)]
pub(crate) struct User {
    #[schema(example = 682167858)]
    pub tg_user_id: i64,
    #[schema(example = 665367136)]
    pub tg_chat_id: i64,
}
