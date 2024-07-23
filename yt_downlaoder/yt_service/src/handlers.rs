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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;
    #[actix_rt::test]
    async fn create_video_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_data: web::Data<AppData> = web::Data::new(AppData { db: pool });
        let uid = web::Path::from(1);
        let request: web::Json<VideoRequest> = web::Json(VideoRequest {
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
        });

        let res = create_video(app_data, uid, request).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_video_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_data: web::Data<AppData> = web::Data::new(AppData { db: pool });
        let vid = web::Path::from("v=dQw4w9WgXcQ".to_string());

        let res = delete_video(app_data, vid).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_all_videos_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_data: web::Data<AppData> = web::Data::new(AppData { db: pool });

        let res = delete_all_videos(app_data).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_all_videos_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_data: web::Data<AppData> = web::Data::new(AppData { db: pool });

        let res = get_all_videos(app_data).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
