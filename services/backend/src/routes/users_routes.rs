use std::sync::Arc;
use axum::{extract::State, routing::{delete, post}, Json};

use crate::models::user_models::User;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        create_user,
        delete_user,
    ),
)]
pub(super) struct UsersApi;

pub(crate) fn routes<T: Clone + Send + Sync + 'static>() -> axum::Router<Arc<T>> {
    axum::Router::new()
        .route("/", post(create_user))
        .route("/", delete(delete_user))
}

#[utoipa::path(
    post,
    path = "",
    description = "Creates new user.",
    request_body = User,
    responses(
        (
            status = 201,
            body = User,
            description = "User created.",
        ),
        (
            status = 409,
            body = AppError,
            description = "User with this id already exists.",
            example = json!({"code": 409, "body": "User with id 665367136 already exists."}),
        ),
        (
            status = 500,
            body = AppError,
            description = "Internal server error.",
            example = json!({"code": 500, "body": "Internal server error."}),
        ),
    ),
)]
async fn create_user<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
    Json(_new_user): Json<User>,
) -> impl axum::response::IntoResponse {
    todo!()
}

#[utoipa::path(
    delete,
    path = "",
    description = "Delete user.",
    request_body = User,
    responses(
        (
            status = 200,
            description = "User successfuly deleted.",
        ),
        (
            status = 404,
            body = AppError,
            description = "User not found.",
            example = json!({"code": 404, "body": "User with id 665367136 doesn't exists."}),
        ),
        (
            status = 500,
            body = AppError,
            description = "Internal server error.",
            example = json!({"code": 500, "body": "Internal server error."}),
        ),
    ),
)]
async fn delete_user<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
    Json(_user): Json<User>,
) -> impl axum::response::IntoResponse {
    todo!()
}
