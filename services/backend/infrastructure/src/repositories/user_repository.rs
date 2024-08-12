use std::sync::Arc;
use domain::{app_error::{AppError, AppResult}, users::{models::entities::{NewUserEntity, UpdateUserEntity, UserEntity}, repository::UserRepository}};
use sqlx::{query, query_as, PgPool};
use super::test_utils::PgRepository;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PgRepository for PostgresUserRepository {
    fn new(pool: Arc<PgPool>) -> Self {
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
    use domain::users::{models::entities::{NewUserEntity, UserEntity}, repository::UserRepository};
    use super::PostgresUserRepository;
    use crate::repositories::test_utils::utils::test_repo;

    #[tokio::test]
    async fn insert_success() {
        test_repo(|repo: PostgresUserRepository| async move {
            let precomputed_new_user = UserEntity {
                id: 1,
                name: "test".to_owned(),
                email: "test".to_owned(),
                password_hash: "test".to_owned(),
                password_salt: "test".to_owned(),
            };

            let requested_new_user = NewUserEntity {
                name: "test".to_owned(),
                email: "test".to_owned(),
                password_hash: "test".to_owned(),
                password_salt: "test".to_owned(),
            };

            let created_new_user = repo.insert(requested_new_user).await.unwrap();

            assert_eq!(precomputed_new_user, created_new_user)
        }).await;
    }

    // async fn inser_fail() {
    //     todo!()
    // }

    // #[tokio::test]
    // async fn read_success() {
    //     let readed_user = get_repository().await.select_by_id(1).await.unwrap();

    //     println!("{}", readed_user.name);

    //     assert!(false)
        
    // }

    // async fn read_fail() {
    //     todo!()
    // }

    // async fn update_success() {
    //     todo!()
    // }

    // async fn update_fail() {
    //     todo!()
    // }

    // async fn delete_success() {
    //     todo!()
    // }

    // async fn delete_fail() {
    //     todo!()
    // }
}
