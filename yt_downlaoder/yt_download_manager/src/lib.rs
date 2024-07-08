pub mod models {
    use actix_web::web;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FetchMeta {
        pub title: String,
        pub url: String,
        pub thumbnail_url: String,
        pub video_id: String,
        pub size: u64,
    }
    impl From<web::Json<FetchMeta>> for FetchMeta {
        fn from(fetched: web::Json<FetchMeta>) -> Self {
            FetchMeta {
                title: fetched.title.clone(),
                url: fetched.url.clone(),
                thumbnail_url: fetched.thumbnail_url.clone(),
                video_id: fetched.video_id.clone(),
                size: fetched.size,
            }
        }
    }
}

pub mod handlers {
    use super::models::FetchMeta;
    use actix_web::{web, HttpResponse};
    use rustube::{url::Url, Video, VideoFetcher};
    use std::fs;
    use std::path::Path;

    //downloads videos to downloads folder and returns json metas data of video
    pub async fn download_video(path: web::Path<String>) -> HttpResponse {
        let path_url = path.into_inner();
        let url = Url::parse(&path_url).unwrap();
        let video = Video::from_url(&url).await.unwrap();
        //TODO: thumbnail is in ARC need to find new method toi handle
        let thumbnail_url = "fill".to_string();
        let video_id = video.id().to_string();
        let title = video.title().to_string();

        //download video to donwloads if it exist, TODO: otherwise create downloads folder
        let download_dir = Path::new("./downloads/");
        let video_file = video
            .best_quality()
            .unwrap()
            .download_to_dir(download_dir)
            .await
            .unwrap();

        //get file size of video
        let size: u64 = video_file.metadata().unwrap().len();

        HttpResponse::Ok().json(FetchMeta {
            title,
            url: path_url,
            thumbnail_url,
            video_id,
            size,
        })
    }

    //deletes a video from downloads folder and retrurns json of completion
    pub async fn delete_video(path: web::Path<String>) -> HttpResponse {
        let video_location = format!("./downloads/{}{}", path.into_inner(), ".mp4");

        let is_deleted = fs::remove_file(Path::new(&video_location)).unwrap();

        HttpResponse::Ok().json(&format!("{:?} was removed", &video_location))
    }
    //deletes all videos in downloads directory and returns json of success
    pub async fn delete_all() -> HttpResponse {
        let download_dir = Path::new("./downloads/");
        let resp = fs::remove_dir_all(download_dir);

        HttpResponse::Ok().json("all videos deleted")
    }
}

#[cfg(test)]
mod tests {
    use super::handlers::{delete_all, delete_video, download_video};
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test, web, App,
    };
    use std::fs;
    use std::path::Path;

    #[actix_rt::test]
    async fn test_downloads_test() {
        let url = web::Path::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string());
        let resp = download_video(url).await;
        assert_eq!(resp.status(), StatusCode::OK);
        match fs::read(Path::new("Never Gonna Give You Up.mp4")) {
            Ok(_) => println!("Video does exist"),
            Err(_) => println!("ERROR Something has gone terribly wrong(check file just in case)"),
        };
    }
    #[actix_rt::test]
    async fn delete_video_test() {
        let title = web::Path::from("Never Gonna Give You Up".to_string());
        let resp = delete_video(title).await;
        assert_eq!(resp.status(), StatusCode::OK);
        match fs::read(Path::new("Never Gonna Give You Up.mp4")) {
            Ok(_) => println!("ERROR video still exist"),
            Err(_) => println!("Success"),
        };
    }
    #[actix_rt::test]
    async fn delete_all_test() {
        let resp = delete_all().await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
