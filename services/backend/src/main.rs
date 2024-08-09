mod domain;

use std::sync::Arc;
use domain::{WebApi, WebContext};

#[cfg(feature = "axum")]
mod axum;
#[cfg(feature = "axum")]
use axum::AxumApi as WebApiImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_service = todo!();
    let topic_service: todo!();
    let subscription_service: todo!();
    
    let ctx = Arc::new(WebContext {
        user_service,
        topic_service,
        subscription_service
    });
    
    WebApiImpl::new("127.0.0.1", 8080).run(ctx).await
}

