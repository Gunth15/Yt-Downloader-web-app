use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tera::Tera;
use yt_web::handlers::general::err_handler;
use yt_web::{routes::*, state::AppData};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Host port from .env file
    let host_port = env::var("HOST_PORT").expect("Host port not set in .env file");
    println!("Listening on port {:?}", &host_port);
    // Database server from .env file
    let db_addr = env::var("DATABASE_URL").expect("Database url is not set in .env file");
    let db_pool = sqlx::postgres::PgPool::connect(&db_addr).await.unwrap();
    let shared_dbpool = web::Data::new(AppData { db: db_pool });
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/html/**/*")).unwrap();
        App::new()
            .app_data(shared_dbpool.clone())
            .app_data(web::Data::new(tera))
            .service(fs::Files::new("/static", ".").show_files_listing())
            .default_service(web::get().to(err_handler))
            .configure(videos::video_routes)
            .configure(user::new_user_routes)
            .configure(user::delete_user_routes)
            .configure(user::update_user_routes)
            .configure(general::general_routes)
    })
    .bind(&host_port)?
    .run()
    .await
}
