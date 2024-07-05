use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoQuery {
    name: String,
    thumbnail_url: Option<String>,
    query_time: Option<NaiveDateTime>,
    user_id: i32,
    video_id: String,
    status: bool,
    size: Option<i32>,
}
impl From<web::Json<VideoQuery>> for VideoQuery {
    fn from(video_query: web::Json<VideoQuery>) -> Self {
        VideoQuery {
            name: video_query.name.clone(),
            thumbnail_url: video_query.thumbnail_url.clone(),
            query_time: video_query.query_time.clone(),
            user_id: video_query.user_id.clone(),
            video_id: video_query.video_id.clone(),
            status: video_query.status,
            size: video_query.size,
        }
    }
}
