use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use yt_download_manager::handlers::{delete_all, delete_video, download_video};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host_port = env::var("HOST_PORT").expect("Host port not configured in .env file");
    println!("listening on port {:?}", &host_port);

    HttpServer::new(move || {
        App::new().service(
            web::scope("")
                .service(web::resource("download/{id}").route(web::post().to(download_video)))
                .service(web::resource("delete/!!").route(web::delete().to(delete_all)))
                .service(web::resource("delete/{title}").route(web::delete().to(delete_video))),
        )
    })
    .bind(&host_port)?
    .run()
    .await
}
