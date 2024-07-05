use crate::dbscript::{create_video_db, delete_all_videos_db, delete_video_db, get_all_videos_db};
use crate::models::VideoQuery;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn create_video(app_state: web::Data<AppState>) -> HttpResponse {}
pub async fn delete_video() {}
pub async fn get_all_videos() {}
pub async fn delete_all_videos() {}
