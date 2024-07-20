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
        pub size: i32,
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
}
