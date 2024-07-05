//redeclarations
pub mod dbscript;
pub mod errors;
pub mod handlers;
pub mod models;

pub mod routes {
    use crate::handlers::{create_video, delete_all_videos, delete_video, get_all_videos};
    use actix_web::web;

    pub fn app_config(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("/video")
                .service(web::resource("").route(web::get().to(get_all_videos)))
                .service(web::resource("/create").route(web::post().to(create_video)))
                .service(web::resource("/delete").route(web::delete().to(delete_all_videos)))
                .service(web::resource("/delete/{video_id}").route(web::delete().to(delete_video))),
        );
    }
}

pub mod state {
    use sqlx::PgPool;

    pub struct AppState {
        pub db: PgPool,
    }
}
