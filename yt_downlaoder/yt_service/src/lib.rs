//redeclrarations
pub mod dbscript;
pub mod handlers;
pub mod ytscripts;
//NOTE update cargo file with newest version of dependencies
pub mod routes {
    use crate::handlers::{create_video, delete_all_videos, delete_video, get_all_videos};
    use actix_web::web;
    pub fn app_config(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("/videos")
                .service(web::resource("").route(web::get().to(get_all_videos)))
                .service(web::resource("/new").route(web::post().to(create_video)))
                .service(web::resource("/delete").route(web::delete().to(delete_all_videos)))
                .service(web::resource("/delete/{video_id}").route(web::delete().to(delete_video))),
        );
    }
}

pub mod models {
    use actix_web::web;
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct VideoQuery {
        pub name: Option<String>,
        pub url: String,
        pub thumbnail_url: Option<String>,
        pub query_time: Option<NaiveDate>,
        pub user_id: i32,
        pub video_id: String,
        pub status: bool,
        pub size: i32,
    }
    impl From<web::Json<VideoQuery>> for VideoQuery {
        fn from(video_query: web::Json<VideoQuery>) -> Self {
            VideoQuery {
                name: video_query.name.clone(),
                url: video_query.url.clone(),
                thumbnail_url: video_query.thumbnail_url.clone(),
                query_time: video_query.query_time.clone(),
                user_id: video_query.user_id,
                video_id: video_query.video_id.clone(),
                status: video_query.status,
                size: video_query.size,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VideoRequest {
        pub url: String,
        pub user_id: String,
    }
}

pub mod state {
    use sqlx::postgres::PgPool;

    struct AppData {
        db: PgPool,
    }
}
