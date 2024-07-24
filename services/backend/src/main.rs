mod config;
mod app;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::App::new().serve().await
}
