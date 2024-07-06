use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use yt_service::{routes::app_config, state::AppData};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Host port from .env file
    let host_port = env::var("HOST_PORT").expect("Host port not set in .env file");
    println!("Listening on port {:?}", &host_port);
    // Database server from .env file
    let db_addr = env::var("DATABASE_URL").expect("Database url is not set in .env file");
    let db_pool = PgPool::connect(&db_addr).await.unwrap();

    let shared_dbpool = web::Data::new(AppData { db: db_pool });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_dbpool.clone())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
