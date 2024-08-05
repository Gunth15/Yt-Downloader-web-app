use crate::models::VideoQuery;
use actix_web::{web, HttpResponse};
use tera::Tera;

pub async fn get_videos_handler(tmpl: web::Data<Tera>) -> HttpResponse {
    let cli = awc::Client::default();

    let req = cli
        .get("127.0.0.1:3000/videos")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let videos: Vec<VideoQuery> =
        serde_json::from_str(&std::str::from_utf8(&req).unwrap()).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("video_list", &videos);
    let ren = tmpl.render("downloads.html", &ctx).unwrap();

    HttpResponse::Ok().body(ren)
}
pub async fn delete_videos_handler(video_id: web::Path<String>) -> HttpResponse {
    let cli = awc::Client::default();
    let url = format!("127.0.0.1:3000/videos/{video_id}");
    let req = cli.delete(url).send().await.unwrap().body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();
    HttpResponse::Ok().body(resp)
}
pub async fn delete_all_videos_handler() -> HttpResponse {
    let cli = awc::Client::default();
    let url = format!("127.0.0.1:3000/videos/delete/ALL!");
    let req = cli.delete(url).send().await.unwrap().body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();

    HttpResponse::Ok().body(resp)
}
