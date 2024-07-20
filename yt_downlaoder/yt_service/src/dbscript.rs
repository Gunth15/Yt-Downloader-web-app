use crate::models::VideoQuery;
use sqlx::postgres::PgPool;

pub async fn create_video_db(pool: &PgPool, query: VideoQuery) -> String {
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
    "Successfully Downloaded".to_string()
}
pub async fn delete_video_db(pool: &PgPool, vid: &str) -> String {}
pub async fn get_video_db(pool: &PgPool, vid: &str) -> VideoQuery {}
pub async fn get_all_videos_db(pool: &PgPool) -> vec<VideoQuery> {}
pub async fn delete_all_videos_db(pool: &PgPool) -> String {}
