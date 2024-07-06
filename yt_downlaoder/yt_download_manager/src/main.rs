use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use yt_download_manager::handlers::{delete_all, delete_video, download_video, get_details};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host_port = env::var("HOST_PORT").expect("Host port not configured in .env file");
    println!("listening on port {:?}", &host_port);

    //TODO add defualt error page
    HttpServer::new(move || {
        App::new().service(
            web::scope("")
                .service(web::resource("download/{url}").route(web::post().to(download_video)))
                .service(web::resource("fetc/{url}").route(web::get().to(get_details)))
                .service(web::resource("delete/!!").route(web::delete().to(delete_all)))
                .service(web::resource("delete/{url}").route(web::delete().to(delete_video))),
        )
    })
    .bind(&host_port)?
    .run()
    .await
}
