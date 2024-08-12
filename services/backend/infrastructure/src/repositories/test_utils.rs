use std::sync::Arc;

#[allow(unused)]
pub trait PgRepository {
    fn new(pool: Arc<sqlx::PgPool>) -> Self;
}

#[cfg(test)]
pub mod utils {
    use std::future::Future;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use super::PgRepository;
    
    pub async fn test_repo<R, F, Fut>(test_fn: F)
    where
        R: PgRepository,
        F: FnOnce(R) -> Fut,
        Fut: Future<Output = ()>,
    {
        let container = Postgres::default().start().await.expect("Can not start container");
        let port = container.get_host_port_ipv4(5432).await.expect("Can not get container port");
        let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    
        let pool = sqlx::PgPool::connect(&url).await.expect("Can not connect to container");
        sqlx::migrate!("../../migrations/migrations").run(&pool).await.expect("Migration fail");

        let repository = <R>::new(std::sync::Arc::new(pool));
        test_fn(repository).await;
    }
}

