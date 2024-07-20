use crate::models::VideoQuery;
use sqlx::postgres::PgPool;

pub async fn create_video_db(pool: &PgPool, query: VideoQuery) -> String {
    let video_query = sqlx::query!(
        "
        INSERT INTO videos 
        (title,url,thumbnail_url,user_id,video_id,size)
        VALUES ($1,$2,$3,$4,$5,$6)
         ",
        query.title,
        query.url,
        query.thumbnail_url,
        query.user_id,
        query.video_id,
        query.size,
    );
    "Successfully Downloaded".to_string()
}
pub async fn delete_video_db(pool: &PgPool, vid: &str) -> String {}
pub async fn get_video_db(pool: &PgPool, vid: &str) -> VideoQuery {}
pub async fn get_all_videos_db(pool: &PgPool) -> vec<VideoQuery> {}
pub async fn delete_all_videos_db(pool: &PgPool) -> String {}
