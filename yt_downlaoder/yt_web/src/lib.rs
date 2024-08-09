//redeclarations
pub mod errors;
pub mod handlers;
pub mod models;
pub mod routes;

pub const SALT: &[u8] = b"HellaSalty";

pub mod state {
    use sqlx::postgres::PgPool;
    pub struct AppData {
        pub db: PgPool,
    }
}

pub mod dbscripts {
    use crate::{
        errors::WebErrors,
        models::{NewUser, UpdateUser, User},
    };
    use sqlx::postgres;

    pub async fn get_user_db(pg_pool: &postgres::PgPool, uname: &str) -> Result<User, WebErrors> {
        Ok(sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            Where username = $1
            ",
            uname
        )
        .fetch_one(pg_pool)
        .await?)
    }

    pub async fn get_userid_db(pg_pool: &postgres::PgPool, uid: i32) -> Result<User, WebErrors> {
        Ok(sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            Where  user_id = $1
            ",
            uid
        )
        .fetch_one(pg_pool)
        .await?)
    }

    pub async fn post_user_db(
        pg_pool: &postgres::PgPool,
        user: NewUser,
    ) -> Result<String, WebErrors> {
        sqlx::query!(
            "
            INSERT INTO users
            (username, password)
            VALUES ($1,$2)
            ",
            user.username,
            user.password
        )
        .execute(pg_pool)
        .await?;

        Ok("User added".to_string())
    }

    pub async fn update_user_db(
        pg_pool: &postgres::PgPool,
        user: UpdateUser,
    ) -> Result<String, WebErrors> {
        sqlx::query!(
            "
            UPDATE users
            SET password = $1
            WHERE username = $2
            ",
            user.new_password,
            user.username
        )
        .execute(pg_pool)
        .await?;

        Ok("Updated".to_string())
    }

    pub async fn delete_user_db(
        pg_pool: &postgres::PgPool,
        uname: &str,
    ) -> Result<String, WebErrors> {
        sqlx::query!(
            "
            DELETE 
            FROM users
            WHERE username = $1
            ",
            uname
        )
        .execute(pg_pool)
        .await?;

        Ok("Deleted".to_string())
    }
}
