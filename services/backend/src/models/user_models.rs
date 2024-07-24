use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")] 
pub(crate) enum UserRole {
    User,
    Admin,
    Owner,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct User {
    pub id: i64,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NewUser {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdateUser {
    name: Option<String>,
    role: Option<UserRole>,
}
