use aide::{axum::{routing::{delete_with, post_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::extract::Path;

pub(crate) fn subscription_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/subscribe/topic/:id", 
            post_with(subscribe, subscribe_docs)
        )
        .api_route("/unsibscribe/topic/:id", 
                delete_with(unsubscribe, unsubscribe_docs)
        )
}

async fn subscribe(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: subscribe"
}

fn subscribe_docs(op: TransformOperation) -> TransformOperation {
    op.description("Subscribe user to topic")
        .tag("Subscription routes")
        .response::<201, String>()
}

async fn unsubscribe(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: unsibscribe"
}

fn unsubscribe_docs(op: TransformOperation) -> TransformOperation {
    op.description("Unsubscribe user from topic")
        .tag("Subscription routes")
        .response::<204, String>()
}
