use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use crate::{models, error_response::AppErrorResponse};

mod users_routes;
mod subscriptions_routes;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Feed621",
        description = "REST API that looks like feed for e621.net",
        version = "1.0.0",
        contact(name = "hash", url = "https://github.com/h4-h/feed621"),
        license(name = "MIT", url = "https://github.com/h4-h/feed621/blob/main/LICENSE"),
    ),
    nest(
        (path = "/api/users", api = users_routes::UsersApi),
        (path = "/api/subscriptions", api = subscriptions_routes::SubscriptionsApi),
    ),
    tags(
        (name = "users_routes", description = "User routes."),
        (name = "subscriptions_routes", description = "Subscription routes."),
    ),

    components(
        schemas(
            AppErrorResponse,
            models::user_models::User,
            models::topic_models::Topic,
            models::topic_models::NewTopic,
            models::topic_models::UpdateTopic,
            models::subscription_models::Subscription,
        ),
    ),
)]
struct ApiDoc;

pub(crate) fn app<S: Clone + Send + Sync + 'static>(state: S) -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/users", users_routes::routes())
        .nest("/api/subscriptions", subscriptions_routes::routes())
        .with_state(state.into())
}

async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    AppErrorResponse::not_found(format!("Page {uri}"))
}
