use crate::models::{VideoQuery, VideoRequest};
use sqlx::postgres::PgPool;

pub async fn create_video_db(pool: &PgPool, query: VideoQuery) -> VideoQuery {
    let video_query = sqlx::query_as!(
        VideoQuery,
        "
        INSERT INTO videos 
        (name,url,thumbnail_url,query_time,user_id,video_id,status,size)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
         ",
        query.name,
        query.thumbnail_url,
        query.query_time,
        query.user_id,
    );
}
pub async fn delete_video_db(pool: PgPool, video: VideoQuery) -> String {}
pub async fn get_video_db(pool: PgPool, url: VideoRequest) -> VideoQuery {}
pub async fn get_all_videos_db(pool: PgPool) -> vec<VideoQuery> {}
pub async fn delete_all_videos_db() -> String {}
