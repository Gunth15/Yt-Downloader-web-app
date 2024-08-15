use crate::errors::YtManErr;
use crate::models::VideoQuery;
use sqlx::postgres::PgPool;

pub async fn create_video_db(pool: &PgPool, query: VideoQuery) -> Result<String, YtManErr> {
    sqlx::query!(
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
    )
    .execute(pool)
    .await?;
    Ok("Successfully Downloaded".to_string())
}
pub async fn delete_video_db(pool: &PgPool, vid: &str) -> Result<String, YtManErr> {
    let deletetion = sqlx::query!(
        "
        DELETE FROM videos
        WHERE video_id = $1
        ",
        vid
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(format!("Deleted {deletetion} video").to_string())
}
pub async fn get_video_db(pool: &PgPool, vid: &str) -> Result<VideoQuery, YtManErr> {
    Ok(sqlx::query_as!(
        VideoQuery,
        "
       SELECT *
       FROM videos
       WHERE video_id = $1
       ",
        vid
    )
    .fetch_one(pool)
    .await?)
}
pub async fn get_all_videos_db(pool: &PgPool) -> Result<Vec<VideoQuery>, YtManErr> {
    Ok(sqlx::query_as!(
        VideoQuery,
        "
        SELECT *
        FROM videos
        "
    )
    .fetch_all(pool)
    .await?)
}
pub async fn delete_all_videos_db(pool: &PgPool) -> Result<String, YtManErr> {
    let deltetion = sqlx::query!(
        "
        DELETE FROM videos
        "
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(format!("{deltetion} videos deleted").to_string())
}
