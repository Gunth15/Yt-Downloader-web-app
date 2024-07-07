use crate::dbscript::{create_video_db, delete_all_videos_db, delete_video_db, get_all_videos_db};
use crate::models::{VideoQuery, VideoRequest};
use crate::state::AppData;
use crate::ytscripts::{delete_all_yt, delete_video_yt, get_details_yt};
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn create_video(
    app_data: web::Data<AppData>,
    path: web::Path<u32>,
    video_request: web::Json<VideoRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let client = awc::Client::default();

    let resp = client
        .get("127.0.0.1:2024")
        .send_json(&video_request)
        .await
        .unwrap()
        .body()
        .await
        .unwrap();
    let resp: VideoQuery = serde_json::from_str(&std::str::from_utf8(&resp).unwrap()).unwrap();
    HttpResponse::Ok().json(resp)
}
pub async fn delete_video() -> HttpResponse {}
pub async fn get_all_videos() -> HttpResponse {}
pub async fn delete_all_videos() -> HttpResponse {}
