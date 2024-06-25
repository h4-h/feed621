use crate::config::Config;

pub(crate) async fn get_postgres_pool(config: &Config) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(config.database_url())
        .await
        .expect("\x1B[0;31m[!] Can not connect to database.\x1B[0m")
}
