pub mod errors;
pub mod handlers;

pub mod models {
    use actix_web::web;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FetchMeta {
        pub title: String,
        pub url: String,
        pub thumbnail_url: String,
        pub video_id: String,
        pub size: i64,
    }
    impl From<web::Json<FetchMeta>> for FetchMeta {
        fn from(fetched: web::Json<FetchMeta>) -> Self {
            FetchMeta {
                title: fetched.title.clone(),
                url: fetched.url.clone(),
                thumbnail_url: fetched.thumbnail_url.clone(),
                video_id: fetched.video_id.clone(),
                size: fetched.size,
            }
        }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteRequest {
        pub video_id: String,
        pub title: String,
    }
    impl From<web::Json<DeleteRequest>> for DeleteRequest {
        fn from(req: web::Json<DeleteRequest>) -> Self {
            DeleteRequest {
                video_id: req.video_id.clone(),
                title: req.title.clone(),
            }
        }
    }
}
