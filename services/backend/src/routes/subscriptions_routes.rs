use std::sync::Arc;
use axum::{extract::{Path, State}, routing::{delete, get, post}, Json};

use crate::models::user_models::User;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        subscribe,
        get_user_subscriptions,
        unsubscribe,
    ),
)]
pub(super) struct SubscriptionsApi;

pub(crate) fn routes<T: Clone + Send + Sync + 'static>() -> axum::Router<Arc<T>> {
    axum::Router::new()
        .route("/:id", post(subscribe))
        .route("/user", get(get_user_subscriptions))
        .route("/:id", delete(unsubscribe))
}

#[utoipa::path(
    post,
    path = "/{id}",
    description = "Add topic to user subscriptions.",
    params(
        (
            "id" = i64,
            example = 1,
            description = "Topic id.",
        ),
    ),
    request_body = User,
    responses(
        (
            status = 200,
            description = "Successful subscribed.",
        ),
        (
            status = 401,
            body = AppError,
            description = "User and chat id's different from DB record.",
            example = json!({"code": 401, "body": "Unauthorized."}),
        ),
        (
            status = 409,
            body = AppError,
            description = "Already subscribed.",
            example = json!({"code": 409, "body": "Already subscribed to topic with id 1."}),
        ),
        (
            status = 500,
            body = AppError,
            description = "Internal server error.",
            example = json!({"code": 500, "body": "Internal server error."}),
        ),
    ),
)]
async fn subscribe<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
    Path(_topic_id): Path<i64>,
    Json(_user): Json<User>,
) -> impl axum::response::IntoResponse {
    todo!()
}

#[utoipa::path(
    get,
    path = "/user",
    description = "Get user subscriptions.",
    request_body = User,
    responses(
        (
            status = 200,
            body = [Subscription],
            description = "User subscriptions.",
        ),
        (
            status = 401,
            body = AppError,
            description = "User and chat id's different from DB record.",
            example = json!({"code": 401, "body": "Unauthorized."}),
        ),
        (
            status = 500,
            body = AppError,
            description = "Internal server error.",
            example = json!({"code": 500, "body": "Internal server error."}),
        ),
    ),
)]
async fn get_user_subscriptions<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
) -> impl axum::response::IntoResponse {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/{id}",
    description = "Remove topic from user subscriptions.",
    params(
        (
            "id" = i64,
            example = 1,
            description = "Topic id.",
        ),
    ),
    request_body = User,
    responses(
        (
            status = 200,
            description = "User successfuly unsubscribed.",
        ),
        (
            status = 401,
            body = AppError,
            description = "User and chat id's different from DB record.",
            example = json!({"code": 401, "body": "Unauthorized."}),
        ),
        (
            status = 404,
            body = AppError,
            description = "User was not subscribed.",
            example = json!({"code": 404, "body": "Can't unsibscribe without subscription to topic with id 1."}),
        ),
        (
            status = 500,
            body = AppError,
            description = "Internal server error.",
            example = json!({"code": 500, "body": "Internal server error."}),
        ),
    ),
)]
async fn unsubscribe<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
    Path(_topic_id): Path<i64>,
    Json(_user): Json<User>,
) -> impl axum::response::IntoResponse {
    todo!()
}
