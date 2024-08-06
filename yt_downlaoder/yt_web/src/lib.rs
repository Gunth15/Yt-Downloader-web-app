//redeclarations
pub mod handlers;
pub mod models;
pub mod routes;

pub const SALT: &str = "HellaSalty";

pub mod state {
    use sqlx::postgres::PgPool;
    pub struct AppData {
        pub db: PgPool,
    }
}

pub mod dbscripts {
    use crate::models::{NewUser, UpdateUser, User};
    use sqlx::postgres;

    pub async fn get_user_db(pg_pool: &postgres::PgPool, uname: &str) -> User {
        sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            Where username = $1
            ",
            uname
        )
        .fetch_one(pg_pool)
        .await
        .unwrap()
    }

    pub async fn get_userid_db(pg_pool: &postgres::PgPool, uid: i32) -> User {
        sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            Where  user_id = $1
            ",
            uid
        )
        .fetch_one(pg_pool)
        .await
        .unwrap()
    }

    pub async fn post_user_db(pg_pool: &postgres::PgPool, user: NewUser) -> String {
        sqlx::query!(
            "
            INSERT INTO users
            (username, password)
            VALUES ($1,$2)
            ",
            user.username,
            user.password
        )
        .fetch_one(pg_pool)
        .await
        .unwrap();

        "User added".to_string()
    }

    pub async fn update_user_db(pg_pool: &postgres::PgPool, user: UpdateUser) -> String {
        sqlx::query!(
            "
            UPDATE users
            SET password = $1
            WHERE username = $2
            ",
            user.new_password,
            user.username
        )
        .fetch_one(pg_pool)
        .await
        .unwrap();

        "Updated".to_string()
    }

    pub async fn delete_user_db(pg_pool: &postgres::PgPool, uname: &str) -> String {
        sqlx::query!(
            "
            DELETE 
            FROM users
            WHERE username = $1
            ",
            uname
        )
        .fetch_one(pg_pool)
        .await
        .unwrap();

        "Deleted".to_string()
    }
}
