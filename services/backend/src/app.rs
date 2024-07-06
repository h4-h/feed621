use std::sync::Arc;
use anyhow::Context;
use tokio::net::TcpListener;
use crate::{config::Config, routes};

pub(crate) struct AppState {

}

impl AppState {
    pub async fn new(_db_pool: sqlx::PgPool) -> anyhow::Result<Self> {
        Ok(Self {

        })
    }
}

pub(crate) struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        let _ = dotenvy::dotenv();
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

        Self {
            config: Config::new(),
        }
    }
    
    pub async fn serve(&self) -> anyhow::Result<()> {
        let listener = self.bind_address().await?;
        let state = self.init_state().await?;
        let router_service = self.setup_router(state).into_make_service();

        axum::serve(listener, router_service).await
            .context("\x1B[0;31mCan not serve.\x1B[0m")
    }

    async fn bind_address(&self) -> anyhow::Result<TcpListener> {
    tokio::net::TcpListener::bind("0.0.0.0:8080").await
        .context("\x1B[0;31mCan not bind address.\x1B[0m")
    }

    async fn init_state(&self) -> anyhow::Result<AppState> {
        let db_pool = self.init_database().await?;
        AppState::new(db_pool).await
    }
    fn setup_router(&self, state: AppState) -> axum::Router {
        axum::Router::new()
            .with_state(Arc::new(state))
            .merge(routes::configure_routes())
    }

    async fn init_database(&self) -> anyhow::Result<sqlx::PgPool> {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(12)
            .connect(self.config.database_url())
            .await
            .context("\x1B[0;31mCan not connect to database.\x1B[0m")
    }
}
