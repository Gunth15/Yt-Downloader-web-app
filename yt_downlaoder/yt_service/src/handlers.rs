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
    let uid = path.into_inner();
    let video_query = VideoQuery::from_meta(download_video_yt(video_request.into()).await, uid);
    let res = create_video_db(&app_data.db, video_query);

    HttpResponse::Ok().json()
}
pub async fn delete_video(app_data: web::Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let vid = path.into_inner();

    let resp = delete_video_yt(&vid).await;
    let resp = delete_video_db(&app_data.db, &vid).await;

    HttpResponse::Ok().json(&resp)
}
pub async fn get_all_videos(app_data: web::Data<AppData>) -> HttpResponse {
    let res = get_all_videos_db(&app_data.db);
    HttpResponse::Ok().json(&res)
}
pub async fn delete_all_videos(app_data: web::Data<AppData>) -> HttpResponse {
    let res = delete_all_videos_db(&app_data.db);
    HttpResponse::Ok().json(&res)
}
