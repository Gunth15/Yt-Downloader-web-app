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
                .service(web::resource("/new/{user_id}").route(web::post().to(create_video)))
                .service(web::resource("/delete").route(web::delete().to(delete_all_videos)))
                .service(web::resource("/delete/{video_id}").route(web::delete().to(delete_video))),
        );
    }
}

pub mod models {
    use actix_web::web;
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FetchMeta {
        pub name: Option<String>,
        pub url: String,
        pub thumbnail_url: Option<String>,
        pub user_id: Option<i32>,
        pub video_id: String,
        pub size: i32,
    }
    impl From<web::Json<FetchMeta>> for FetchMeta {
        fn from(fetched: web::Json<FetchMeta>) -> Self {
            FetchMeta {
                name: fetched.name.clone(),
                url: fetched.url.clone(),
                thumbnail_url: fetched.thumbnail_url.clone(),
                user_id: fetched.user_id,
                video_id: fetched.video_id.clone(),
                size: fetched.size,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct VideoQuery {
        pub name: Option<String>,
        pub url: String,
        pub thumbnail_url: Option<String>,
        pub query_time: Option<NaiveDate>,
        pub user_id: i32,
        pub video_id: String,
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
                size: video_query.size,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VideoRequest {
        pub url: String,
    }
    impl From<web::Json<VideoRequest>> for VideoRequest {
        fn from(req: web::Json<VideoRequest>) -> Self {
            VideoRequest {
                url: req.url.clone(),
            }
        }
    }
}

pub mod state {
    use sqlx::postgres::PgPool;

    pub struct AppData {
        pub db: PgPool,
    }
}
