use crate::models::DeleteRequest;

use super::errors::YtDlErrors;
use super::models::FetchMeta;
use actix_web::{web, HttpResponse};
use pytube_wrpr;
use std::env;
use std::fmt::format;
use std::fs;

//downloads videos to downloads folder and returns json metas data of video
pub async fn download_video(path: web::Path<String>) -> Result<HttpResponse, YtDlErrors> {
    let id = path.into_inner();
    let url = format!("youtube.com/watch?v={id}");
    //query containts image url and title of file
    let mut dir = env::current_dir().expect("could not find path");
    dir.push("downloads");

    let mut query = pytube_wrpr::download_n_fetch(&url, dir.to_str().unwrap())?;

    let thumbnail_url = query.remove(0);
    let title = {
        match query.remove(0).split_once(".") {
            Some((title, _mp4)) => title.to_string(),
            None => panic!("Fatal error occured"),
        }
    };

    //get size of video
    dir.push(&format!("{title}.mp4"));
    let file_name = dir.to_str().unwrap();
    println!("Downloaded video {}", file_name);

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
pub async fn delete_video(
    delete_req: web::Json<DeleteRequest>,
) -> Result<HttpResponse, YtDlErrors> {
    let delete_req = DeleteRequest::from(delete_req);

    let mut dir = env::current_dir().expect("could not find path");
    dir.push("downloads");
    dir.push(&format!("{}.mp4", delete_req.title));
    let video_location = dir.to_str().unwrap();
    fs::remove_file(video_location).map_err(|_err| {
        YtDlErrors::FileError(format!(
            "{video_location} was not found or download directory does not exist"
        ))
    })?;

    println!("deleted video at {:?}", video_location);
    Ok(HttpResponse::Ok().json(&format!("{:?} was removed", video_location)))
}
//deletes all videos in downloads directory and returns json of success
pub async fn delete_all() -> Result<HttpResponse, YtDlErrors> {
    let mut dir = env::current_dir().expect("could not find path");
    dir.push("downloads");
    let download_dir = dir.to_str().unwrap();

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
        let delete_request: web::Json<DeleteRequest> = web::Json(DeleteRequest {
            title: "Rick Astley - Never Gonna Give You Up (Official Music Video)".to_string(),
            video_id: "dQw4w9WgXcQ".to_string(),
        });

        let resp = delete_video(delete_request).await.unwrap();

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
