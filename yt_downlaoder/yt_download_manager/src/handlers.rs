use super::errors::YtDlErrors;
use super::models::FetchMeta;
use actix_web::{web, HttpResponse};
use rustube::{url::Url, Video};
use std::fs;
use std::path::Path;

//downloads videos to downloads folder and returns json metas data of video
pub async fn download_video(path: web::Path<String>) -> Result<HttpResponse, YtDlErrors> {
    let path_url = path.into_inner();
    let url = Url::parse(&path_url)?;

    let video = Video::from_url(&url).await?;
    //TODO: thumbnail is in ARC need to find new method to handle
    let thumbnail_url = video.video_info()
    let video_id = video.id().to_string();
    let title = video.title().to_string();

    //download video to donwloads if it exist
    let download_dir = Path::new("./downloads/");
    let video_file = video
        .best_quality()
        .ok_or(YtDlErrors::RtError("Could not find stream".to_string()))?
        .download_to_dir(download_dir)
        .await?;

    //get file size of video
    let size: u64 = video_file.metadata().unwrap().len();

    Ok(HttpResponse::Ok().json(FetchMeta {
        title,
        url: path_url,
        thumbnail_url,
        video_id,
        size,
    }))
}

//deletes a video from downloads folder and retrurns json of completion
pub async fn delete_video(path: web::Path<String>) -> Result<HttpResponse, YtDlErrors> {
    let video_location = format!("./downloads/{}{}", path.into_inner(), ".mp4");

    fs::remove_file(Path::new(&video_location)).map_err(|_err| {
        YtDlErrors::FileError("File was not found or download directory does not exist".to_string())
    })?;

    Ok(HttpResponse::Ok().json(&format!("{:?} was removed", &video_location)))
}
//deletes all videos in downloads directory and returns json of success
pub async fn delete_all() -> Result<HttpResponse, YtDlErrors> {
    let download_dir = Path::new("./downloads/");
    fs::remove_dir_all(download_dir).map_err(|_err| {
        YtDlErrors::FileError("Download directory could not be found no files deleted".to_string())
    })?;

    Ok(HttpResponse::Ok().json("all videos deleted"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, web};
    use std::fs;
    use std::path::Path;

    #[actix_rt::test]
    async fn test_downloads_test() {
        let url = web::Path::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string());
        let resp = download_video(url).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        match fs::read(Path::new("Never Gonna Give You Up.mp4")) {
            Ok(_) => println!("Video does exist"),
            Err(_) => println!("ERROR Something has gone terribly wrong(check file just in case)"),
        };
    }
    #[actix_rt::test]
    async fn delete_video_test() {
        let title = web::Path::from("Never Gonna Give You Up".to_string());
        let resp = delete_video(title).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        match fs::read(Path::new("Never Gonna Give You Up.mp4")) {
            Ok(_) => println!("ERROR video still exist"),
            Err(_) => println!("Success"),
        };
    }
    #[actix_rt::test]
    async fn delete_all_test() {
        let resp = delete_all().await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
