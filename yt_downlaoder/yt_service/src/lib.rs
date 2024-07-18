//redeclrarations
pub mod dbscript;
pub mod handlers;
pub mod models;
pub mod ytscripts;

//NOTE: update cargo file with newest version of dependencies
pub mod routes {
    use crate::handlers::{create_video, delete_all_videos, delete_video, get_all_videos};
    use actix_web::web;
    pub fn app_config(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("/videos")
                .service(web::resource("").route(web::get().to(get_all_videos)))
                .service(web::resource("/new/{user_id}").route(web::post().to(create_video)))
                .service(web::resource("/delete").route(web::delete().to(delete_all_videos)))
                .service(web::resource("/delete/{video_id}").route(web::delete().to(delete_video))),
        );
    }
}

pub mod state {
    use sqlx::postgres::PgPool;

    pub struct AppData {
        pub db: PgPool,
    }
}
