use std::sync::Arc;
use axum::{extract::{Path, State}, routing::get};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(test_get, echo),
)]
pub(super) struct TestApi;

pub(crate) fn routes<T: Clone + Send + Sync + 'static>() -> axum::Router<Arc<T>> {
    axum::Router::new()
        .route("/", get(test_get))
        .route("/:echo_text", get(echo))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = 200, description = "Indicates that app work.", body = String)),
)]
async fn test_get<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
) -> impl axum::response::IntoResponse {
    "OwO it seems working...".to_string()
}


#[utoipa::path(
    get,
    path = "/{echo_text}",
    params(("echo_text" = String, description = "Text that will be returned in the body.")),
    responses((status = 200, description = "Returns {echo_text} in the body.", body = String)),
)]
async fn echo<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
    Path(echo_text): Path<String>,
) -> impl axum::response::IntoResponse {
    echo_text
}
