use crate::dbscript::{create_video_db, delete_all_videos_db, delete_video_db, get_all_videos_db};
use crate::models::{VideoQuery, VideoRequest};
use crate::state::AppData;
use crate::ytscripts::{delete_all_yt, delete_video_yt, download_video_yt};
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn create_video(
    app_data: web::Data<AppData>,
    path: web::Path<u32>,
    video_request: web::Json<VideoRequest>,
) -> HttpResponse {
}
pub async fn delete_video() -> HttpResponse {}
pub async fn get_all_videos() -> HttpResponse {}
pub async fn delete_all_videos() -> HttpResponse {}
