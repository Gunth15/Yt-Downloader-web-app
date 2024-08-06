use crate::dbscripts::get_userid_db;
use crate::models::{DlRequest, VideoQuery, VideoRequest};
use crate::state::AppData;
use actix_web::{web, HttpResponse};
use argon2::{password_hash::Salt, Argon2, PasswordHasher};
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

    HttpResponse::Ok().content_type("text/html").body(ren)
}
pub async fn delete_video_handler(video_id: web::Path<String>) -> HttpResponse {
    let cli = awc::Client::default();
    let url = format!("127.0.0.1:3000/videos/{video_id}");
    let req = cli.delete(url).send().await.unwrap().body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();
    HttpResponse::Ok().content_type("text/html").body(resp)
}
pub async fn delete_all_videos_handler() -> HttpResponse {
    let cli = awc::Client::default();
    let url = format!("127.0.0.1:3000/videos/delete/ALL!");
    let req = cli.delete(url).send().await.unwrap().body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();

    HttpResponse::Ok().content_type("text/html").body(resp)
}
pub async fn post_video_handler(
    to_dl: web::Form<DlRequest>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<Tera>,
) -> HttpResponse {
    let video_req = VideoRequest {
        url: to_dl.url.clone(),
    };

    let uid = to_dl.id;

    let config = Argon2::default();
    let hash = config
        .hash_password(
            to_dl.password.as_bytes(),
            Salt::from_b64(crate::SALT).unwrap(),
        )
        .unwrap()
        .to_string();

    // verify password
    let user = get_userid_db(&app_data.db, uid).await;
    let resp = if &hash == &user.password {
        let cli = awc::Client::default();
        let url = format!("127.0.0.1:3000/videos/new/{uid}");
        let req = cli
            .post(url)
            .send_json(&video_req)
            .await
            .unwrap()
            .body()
            .await
            .unwrap();
        std::str::from_utf8(&req).unwrap().to_string()
    } else {
        let mut ctx = tera::Context::default();
        ctx.insert("Error", "Wrong password");
        ctx.insert("logged_in", &true);

        tmpl.render("landing.html", &ctx).unwrap()
    };
    HttpResponse::Ok().content_type("text/html").body(resp)
}
