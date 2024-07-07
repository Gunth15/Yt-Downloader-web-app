pub mod handlers {
    use actix_web::{web, HttpResponse};
    use rustube::{Video, VideoFetcher};
    use std::fs;
    use std::path::Path;
    use yt_service::models::FetchMeta;

    //downloads videos to downloads folder and returns json data of completion or error
    pub fn download_video(path: web::Path<String>) {
        let url = path.into_inner();
        let video = Video::from_url(&url).await.unwrap();
        let thumbnail_url = video.video_details().thumbnails.pop().unwrap().url.clone();
        let video_id = video.id().to_string();
        let title = video.title().to_string();

        //download video to donwloads if it exist, TODO: otherwise create downloads folder
        let download_dir = Path::new("/downloads/");
        let video_file = video
            .best_quality()
            .unwrap()
            .download_to_dir(download_dir)
            .await
            .unwrap();

        //get file size of video
        let size: u64 = video_file.metadata().unwrap().len();

        FetchMeta {
            title,
            url,
            thumbnail_url,
            video_id,
            size,
        }
    }

    //deletes a video from downloads folder and retrurns json based on success or error
    pub fn delete_video() {}
    //deletes all videos in downloads directory and returns json based on success or error
    pub fn delete_all() {}
}
