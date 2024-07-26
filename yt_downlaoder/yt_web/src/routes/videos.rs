use crate::handlers::*;
use actix_web::web;

pub fn video_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("videos")
            .route("/downloads", web::get().to(get_videos_handler))
            .route("/delete/{video_id}", web::delete().to(delete_video_hadler))
            .route(
                "/deletes/ALL!VIDEOS!",
                web::delete().to(delete_all_videos_handler),
            ),
    );
}
