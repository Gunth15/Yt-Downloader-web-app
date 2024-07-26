//redeclarations
pub mod handlers;
pub mod models;
pub mod routes;

pub mod state {
    use sqlx::postgres::PgPool;
    pub struct AppData {
        pub db: PgPool,
    }
}
