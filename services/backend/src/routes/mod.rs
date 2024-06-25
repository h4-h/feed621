use crate::response::AppResponse;

mod test_routes;

pub(crate) fn app<T: Clone + Send + Sync + 'static>(state: T) -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .merge(test_routes::routes::<T>())
        .with_state(state.into())
}

async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    AppResponse::not_found(format!("Page {uri}"))
}
