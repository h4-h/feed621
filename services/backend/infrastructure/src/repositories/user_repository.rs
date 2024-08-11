use std::sync::Arc;
use domain::{app_error::{AppError, AppResult}, users::{models::entities::{NewUserEntity, UpdateUserEntity, UserEntity}, repository::UserRepository}};
use sqlx::{query, query_as, PgPool};

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn insert(&self, new_user: NewUserEntity) -> AppResult<UserEntity> {
       query_as!(UserEntity,
            r#"
            INSERT INTO users
                (name, email, password_hash, password_salt)
            VALUES
                ($1, $2, $3, $4)
            RETURNING *
            "#,
            new_user.name,
            new_user.email,
            new_user.password_hash,
            new_user.password_salt
        ).fetch_one(self.pool.as_ref()).await
            .map_err(|e| AppError::Internal(e.into())) // FIXME: change error.
    }

    async fn select_by_id(&self, user_id: i64) -> AppResult<UserEntity> {
        query_as!(UserEntity, 
            r#"
                SELECT * FROM users
                    WHERE id = $1
            "#,
            user_id
        ).fetch_one(self.pool.as_ref()).await
            .map_err(|e| AppError::Internal(e.into())) // FIXME: change error.
    }

    async fn update(&self, update_user: UpdateUserEntity) -> AppResult<UserEntity> {
        query_as!(UserEntity,
            r#"
            UPDATE users SET
                name = COALESCE($2, name),
                email = COALESCE($3, email),
                password_hash = COALESCE($4, password_hash),
                password_salt = COALESCE($5, password_salt)
            WHERE id = $1
                RETURNING *
            "#,
            update_user.id,
            update_user.name,
            update_user.email,
            update_user.password_hash,
            update_user.password_salt
        ).fetch_one(self.pool.as_ref()).await
            .map_err(|e| AppError::Internal(e.into())) // FIXME: change error.
    }

    async fn delete(&self, user_id: i64) -> AppResult<()> {
        query!(r#"DELETE FROM users WHERE id = $1"#, user_id)
            .execute(self.pool.as_ref()).await
            .map_err(|e| AppError::Internal(e.into())) // FIXME: change error.
            .map(|_| ()) // map PgQueryResult to Unit type
    }
}

#[cfg(test)]
mod test {
    use domain::users::repository::UserRepository;
    use testcontainers::{runners::AsyncRunner, ContainerRequest, ImageExt};
    use testcontainers_modules::postgres::Postgres;
    use super::PostgresUserRepository;

    fn init_container() -> ContainerRequest<Postgres> {
        Postgres::default()
            .with_env_var("POSTGRES_DB", "test_db_feed621")
            .with_env_var("POSTGRES_USER", "root")
            .with_env_var("POSTGRES_PASSWORD", "root")
    }

    async fn get_repository() -> impl UserRepository {
        let container = init_container().start().await.expect("can not launch container");
        let host = container.get_host().await.expect("can not get container host");
        let port = container.get_host_port_ipv4(5432).await.expect("can not get container port");
        let db_url = format!("postgres://root:root@{host}:{port}/test_db_feed621");

        let pool = sqlx::PgPool::connect(&db_url).await.expect("can not connect to database");
        PostgresUserRepository::new(std::sync::Arc::new(pool))
    }

    #[tokio::test]
    async fn insert_success() {
        todo!()
    }

    async fn inser_fail() {
        todo!()
    }

    async fn read_success() {
        todo!()
    }

    async fn read_fail() {
        todo!()
    }

    async fn update_success() {
        todo!()
    }

    async fn update_fail() {
        todo!()
    }

    async fn delete_success() {
        todo!()
    }

    async fn delete_fail() {
        todo!()
    }
}
