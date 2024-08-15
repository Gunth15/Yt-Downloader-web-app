pub use crate::models::{FetchMeta, VideoRequest};
use crate::{errors::YtManErr, models::DeleteRequest};
use dotenv::dotenv;
use std::env;

//make request to yt download manager and retrieve title and  retrieve the json meta data
pub async fn download_video_yt(video_req: VideoRequest) -> Result<FetchMeta, YtManErr> {
    dotenv().ok();

    let url = video_req.url;
    let id = url.split_once("v=").unwrap().1.to_string();
    let client_ip = env::var("CLIENT_PORT").unwrap();
    let client_url = format!("http://{client_ip}/download/{id}");
    println!("{client_url}");

    let client = awc::Client::default();

    let resp = client
        .post(client_url)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await?
        .body()
        .await
        .unwrap();

    let meta: FetchMeta = serde_json::from_str(&std::str::from_utf8(&resp).unwrap()).unwrap();

    Ok(meta)
}
//delete video and fetch json response of completion
pub async fn delete_video_yt(delete_request: DeleteRequest) -> Result<String, YtManErr> {
    dotenv().ok();

    let vid = &delete_request.video_id;
    let client_ip = env::var("CLIENT_PORT").unwrap();
    let client_url = format!("http://{client_ip}/delete/{vid}");

    let client = awc::Client::default();
    let resp = client
        .delete(client_url)
        .send_json(&delete_request)
        .await?
        .body()
        .await
        .unwrap();

    Ok(serde_json::from_str(&std::str::from_utf8(&resp).unwrap()).unwrap())
}
//deleta all videos and take json completion response
pub async fn delete_all_yt() -> Result<String, YtManErr> {
    dotenv().ok();

    let client_ip = env::var("CLIENT_PORT").unwrap();
    let client_url = format!("http://{client_ip}/delete/!!");

    let client = awc::Client::default();
    let resp = client
        .delete(client_url)
        .send()
        .await?
        .body()
        .await
        .unwrap();

    Ok(serde_json::from_str(&std::str::from_utf8(&resp).unwrap()).unwrap())
}
