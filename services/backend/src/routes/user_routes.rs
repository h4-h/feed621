use aide::{axum::{routing::{get_with, post_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::extract::Path;

pub(crate) fn user_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/users", 
            post_with(register, register_docs)
        )
        .api_route("/users/:id/subscriptions", 
            get_with(get_user_subscriptions, get_user_subscriptions_docs)
        )
        .api_route("/users/:id", 
            get_with(get_user, get_user_docs)
            .patch_with(update_user, update_user_docs)
            .delete_with(delete_user, delete_user_docs)
        )
}

async fn register(
    
) -> impl IntoApiResponse {
    "NYI: user registration"
}

fn register_docs(op: TransformOperation) -> TransformOperation {
    op.description("Register new user")
        .tag("User routes")
        .response::<201, String>()
}

async fn get_user(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: get user"
}

fn get_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get user information")
        .tag("User routes")
        .response::<200, String>()
}

async fn get_user_subscriptions (
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: get user subscriptions"
}

fn get_user_subscriptions_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get user subscriptions")
        .tag("User routes")
        .response::<200, String>()
}

async fn update_user(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: update user"
}

fn update_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update user information")
        .tag("User routes")
        .response::<202, String>()
}

async fn delete_user(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: delete user"
}

fn delete_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete user by id")
        .tag("User routes")
        .response::<204, String>()
}
