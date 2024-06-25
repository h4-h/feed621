use std::sync::Arc;
use axum::{extract::{Path, State}, routing::{get, post}};

pub(crate) fn routes<T: Clone + Send + Sync + 'static>() -> axum::Router<Arc<T>> {
    axum::Router::new()
        .route("/", get(test_get))
}

async fn test_get<T: Clone + Send + Sync + 'static>(
    State(_state): State<Arc<T>>,
) -> impl axum::response::IntoResponse {
    "route work.".to_string()
}
