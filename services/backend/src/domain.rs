use std::sync::Arc;

use domain::{subscriptions::repository::SubscriptionRepository, topics::service::TopicService, users::service::UserService};

/// This is `State` / `Context` / `AppData` and other names that mean shared state object.
pub struct WebContext {
    pub user_service: Arc<dyn UserService>,
    pub topic_service: Arc<dyn TopicService>,
    pub subscription_service: Arc<dyn SubscriptionRepository>,
}

/// See [`Self::get_server_info`].
pub trait ServerInfo: Send + Sync {
    const FRAMEWORK: &'static str;
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Returns `{pkg_version}-{server_framework}` string.
    ///
    /// Note that all adapters should respond this string for `GET /info` request.
    fn get_server_info() -> String {
        format!("{}-{}", Self::VERSION, Self::FRAMEWORK)
    }
}

/// Contract for any server-framework adapter.
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait WebApi: Send + Sync where Self: ServerInfo {
    /// Factory function.
    fn new(addr: &str, port: u16) -> Self;

    /// Method for launching content serving.
    async fn run(&self, ctx: Arc<WebContext>) -> Result<(), Box<dyn std::error::Error>>;
}
