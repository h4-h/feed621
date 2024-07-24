use axum::{extract::Path, response::IntoResponse, routing::post};

pub(crate) fn subscription_routes() -> axum::Router {
    axum::Router::new()
        .route("/subscribe/topic/:id", post(subscribe))
        .route("/unsubscribe/topic/:id", post(unsubscribe))
}

async fn subscribe(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: subscribe"
}

async fn unsubscribe(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: unsibscribe"
}
