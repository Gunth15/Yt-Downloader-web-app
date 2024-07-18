use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchMeta {
    pub title: String,
    pub url: String,
    pub thumbnail_url: String,
    pub video_id: String,
    pub size: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VideoQuery {
    pub title: String,
    pub url: String,
    pub thumbnail_url: String,
    pub query_time: Option<NaiveDate>,
    pub user_id: u32,
    pub video_id: String,
    pub size: u64,
}
impl From<web::Json<VideoQuery>> for VideoQuery {
    fn from(video_query: web::Json<VideoQuery>) -> Self {
        VideoQuery {
            title: video_query.title.clone(),
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
