use axum::{extract::Path, response::IntoResponse, routing::{get, post}, Json};
use crate::models::user_models::{NewUser, User, UserRole};

pub(crate) fn user_routes() -> axum::Router {
    axum::Router::new()
        .route("/users", post(register))
        .route("/users/:id/subscriptions", get(get_user_subscriptions))
        .route("/users/:id", 
            get(get_user)
            .patch(update_user)
            .delete(delete_user)
        )
}

async fn register(
    Json(_new_user): Json<NewUser>,
) -> impl IntoResponse {
    Json(User {
        id: 1,
        name: "hash".into(),
        role: UserRole::Owner,
    })
}

async fn get_user(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    Json(User {
        id: 1,
        name: "hash".into(),
        role: UserRole::Owner,
    })
}

async fn get_user_subscriptions (
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: get user subscriptions"
}

async fn update_user(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: update user"
}

async fn delete_user(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: delete user"
}
