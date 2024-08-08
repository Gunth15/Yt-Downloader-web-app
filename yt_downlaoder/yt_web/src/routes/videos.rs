use crate::handlers::videos::*;
use actix_web::web;

pub fn video_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/videos")
            .route("/downloads", web::get().to(get_videos_handler))
            .route("/delete/{video_id}", web::post().to(delete_video_handler))
            .route(
                "/delete/ALL/!VIDEOS!",
                web::post().to(delete_all_videos_handler),
            )
            .route("/download", web::post().to(post_video_handler)),
    );
}
