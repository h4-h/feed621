mod config;
mod app;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::App::new().serve().await
}
