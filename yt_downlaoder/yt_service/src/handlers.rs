use crate::dbscript::{create_video_db, delete_all_videos_db, delete_video_db, get_all_videos_db};
use crate::errors::YtManErr;
use crate::models::{VideoQuery, VideoRequest};
use crate::state::AppData;
use crate::ytscripts::{delete_all_yt, delete_video_yt, download_video_yt};
use actix_web::{web, HttpResponse};

pub async fn create_video(
    app_data: web::Data<AppData>,
    path: web::Path<i32>,
    video_request: web::Json<VideoRequest>,
) -> Result<HttpResponse, YtManErr> {
    let uid = path.into_inner();
    let meta = download_video_yt(video_request.into()).await.unwrap();
    let video_query = VideoQuery::from_meta(meta, uid);
    let res = create_video_db(&app_data.db, video_query).await.unwrap();

    Ok(HttpResponse::Ok().json(&res))
}
pub async fn delete_video(
    app_data: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<HttpResponse, YtManErr> {
    let vid = path.into_inner();

    let _resp = delete_video_yt(&vid).await.unwrap();
    let resp = delete_video_db(&app_data.db, &vid).await.unwrap();

    Ok(HttpResponse::Ok().json(&resp))
}
pub async fn get_all_videos(app_data: web::Data<AppData>) -> Result<HttpResponse, YtManErr> {
    let res = get_all_videos_db(&app_data.db).await.unwrap();
    Ok(HttpResponse::Ok().json(&res))
}
pub async fn delete_all_videos(app_data: web::Data<AppData>) -> Result<HttpResponse, YtManErr> {
    let _res = delete_all_yt().await.unwrap();
    let res = delete_all_videos_db(&app_data.db).await.unwrap();
    Ok(HttpResponse::Ok().json(&res))
}
