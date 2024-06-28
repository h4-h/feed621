mod config;
mod error_response;
mod database;
mod models;
mod routes;

#[derive(Clone)]
pub(crate) struct AppState {
    
}

#[tokio::main]
async fn main() {
    println!("\x1B[0;32m[+] Starting...\x1B[0m");
    
    let _ = dotenvy::dotenv();
    let config = config::Config::new();

    let db_pool = database::get_postgres_pool(&config).await;
    println!("\x1B[0;32m[+] Database connection estabileshed.\x1B[0m");

    let listener = tokio::net::TcpListener::bind(config.service_addr()).await
        .unwrap_or_else(|_| panic!("\x1B[0;31m[!] Can not bind address.\x1B[0m"));

    let state = AppState {

    };
    
    axum::serve(listener, routes::app(state).into_make_service()).await
        .unwrap_or_else(|_| panic!("\x1B[0;31m[!] Can not serve.\x1B[0m"));
    println!("\x1B[0;32m[+] Listening on http://{}\x1B[0m", config.service_addr());
}
