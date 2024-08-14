use super::errors::YtDlErrors;
use super::models::FetchMeta;
use actix_web::{web, HttpResponse};
use pytube_wrpr;
use std::env;
use std::fs;

//downloads videos to downloads folder and returns json metas data of video
pub async fn download_video(path: web::Path<String>) -> Result<HttpResponse, YtDlErrors> {
    let id = path.into_inner();
    let url = format!("youtube.com/watch?v={id}");
    //query containts image url and title of file
    let mut query = pytube_wrpr::download_n_fetch(&url)?;

    let thumbnail_url = query.remove(0);
    let title = {
        match query.remove(0).split_once(".") {
            Some((title, _mp4)) => title.to_string(),
            None => panic!("Fatal error occured"),
        }
    };

    //get size of video
    let dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_err| YtDlErrors::DlError("Directoty Missing".to_string()))?;
    let file_name = format!("{dir}/downloads/{title}.mp4");
    println!("Downloaded video {}", &file_name);

    let size = fs::metadata(file_name)
        .map_err(|_err| YtDlErrors::DlError("File Missing".to_string()))?
        .len();

    Ok(HttpResponse::Ok().json(FetchMeta {
        title: title.to_string(),
        url: url.to_string(),
        thumbnail_url,
        video_id: id.to_string(),
        size: i64::try_from(size).expect("Youtube video too large buffer overflow (Design error)"),
    }))
}

//deletes a video from downloads folder and retrurns json of completion
pub async fn delete_video(path: web::Path<String>) -> Result<HttpResponse, YtDlErrors> {
    let dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_err| YtDlErrors::DlError("Directoty Missing".to_string()))?;
    let video_location = format!("{dir}/downloads/{}{}", path.into_inner(), ".mp4");

    fs::remove_file(&video_location).map_err(|_err| {
        YtDlErrors::FileError("File was not found or download directory does not exist".to_string())
    })?;

    println!("deleted video at {:?}", &video_location);
    Ok(HttpResponse::Ok().json(&format!("{:?} was removed", &video_location)))
}
//deletes all videos in downloads directory and returns json of success
pub async fn delete_all() -> Result<HttpResponse, YtDlErrors> {
    let dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_err| YtDlErrors::DlError("Directoty Missing".to_string()))?;
    let download_dir = format!("{dir}/downloads/");

    fs::remove_dir_all(download_dir).map_err(|_err| {
        YtDlErrors::FileError("Download directory could not be found no files deleted".to_string())
    })?;

    println!("deleted all videos");
    Ok(HttpResponse::Ok().json("all videos deleted"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, web};
    use std::fs;

    #[actix_rt::test]
    async fn test_downloads_test() {
        let url = web::Path::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string());
        let resp = download_video(url).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let download_dir = format!("{dir}/downloads");
        match fs::read(
            "{download_dir}/Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
        ) {
            Ok(_) => println!("Video does exist"),
            Err(_) => println!("ERROR Something has gone terribly wrong(check file just in case)"),
        };
    }
    #[actix_rt::test]
    async fn delete_video_test() {
        let title = web::Path::from(
            "Rick Astley - Never Gonna Give You Up (Official Music Video)".to_string(),
        );
        let resp = delete_video(title).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let download_dir = format!("{dir}/downloads");
        match fs::read(
            "{download_dir}/Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
        ) {
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
