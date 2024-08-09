use std::{net::IpAddr, sync::Arc};
use crate::domain::{ServerInfo, WebApi, WebContext};

mod routes;

/// Helper wrapper for state extraction
pub type ContentExtension = axum::Extension<Arc<WebContext>>;

/// Axum adapter for web-server.
pub struct AxumApi {
    addr: (IpAddr, u16),
}

impl ServerInfo for AxumApi {
    const FRAMEWORK: &'static str = "axum";
}

#[async_trait::async_trait]
impl WebApi for AxumApi {
    fn new(addr: &str, port: u16) -> Self {
        let addr: IpAddr = addr.parse().expect("Invalid server address");

        Self {
            addr: (addr, port),
        }
    }

    async fn run(&self, ctx: Arc<WebContext>) -> Result<(), Box<dyn std::error::Error>> {
        let router = axum::Router::new()
            .merge(routes::app_router())
            .layer(axum::Extension(ctx));

        let listener = tokio::net::TcpListener::bind(self.addr).await
            .unwrap_or_else(|_| panic!("Can not bind server address ({}:{})", self.addr.0, self.addr.1));

        axum::serve(listener, router).await
            .map_err(|e| e.into())
    }
}
