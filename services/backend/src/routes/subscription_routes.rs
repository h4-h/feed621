use aide::{axum::{routing::post_with, ApiRouter, IntoApiResponse}, transform::TransformOperation};

pub(crate) fn subscription_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/subscribe", 
            post_with(subscribe, subscribe_docs)
                .delete_with(unsubscribe, unsubscribe_docs)
        )
}

async fn subscribe(

) -> impl IntoApiResponse {
    "NYI: subscribe"
}

fn subscribe_docs(op: TransformOperation) -> TransformOperation {
    op.description("Subscribe user to topic")
        .tag("Subscription routes")
        .response::<201, String>()
}

async fn unsubscribe(

) -> impl IntoApiResponse {
    "NYI: unsibscribe"
}

fn unsubscribe_docs(op: TransformOperation) -> TransformOperation {
    op.description("Unsubscribe user from topic")
        .tag("Subscription routes")
        .response::<204, String>()
}
