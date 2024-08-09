use crate::dbscripts::get_userid_db;
use crate::errors::WebErrors;
use crate::models::{DlRequest, VideoQuery, VideoRequest};
use crate::state::AppData;
use actix_web::{web, HttpResponse};
use argon2::{self, Config};
use dotenv::dotenv;
use std::env;
use tera::Tera;

pub async fn get_videos_handler(tmpl: web::Data<Tera>) -> Result<HttpResponse, WebErrors> {
    dotenv().ok();

    let cli = awc::Client::default();

    let client_port = env::var("CLIENT_PORT").expect("Client port not set in .env file");

    let req = cli
        .get(format!("{client_port}/videos"))
        .send()
        .await?
        .body()
        .await
        .unwrap();

    let videos: Vec<VideoQuery> =
        serde_json::from_str(&std::str::from_utf8(&req).unwrap()).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("video_list", &videos);
    let ren = tmpl.render("downloads.html", &ctx).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(ren))
}
pub async fn delete_video_handler(video_id: web::Path<String>) -> Result<HttpResponse, WebErrors> {
    dotenv().ok();

    let cli = awc::Client::default();
    let client_port = env::var("CLIENT_PORT").expect("Client port not set in .env file");
    let url = format!("{client_port}/videos/{video_id}");
    let req = cli.delete(url).send().await?.body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn delete_all_videos_handler() -> Result<HttpResponse, WebErrors> {
    dotenv().ok();

    let cli = awc::Client::default();
    let client_port = env::var("CLIENT_PORT").expect("Client port not set in .env file");
    let url = format!("{client_port}/videos/delete/ALL!");
    let req = cli.delete(url).send().await.unwrap().body().await.unwrap();

    let resp = std::str::from_utf8(&req).unwrap().to_string();

    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn post_video_handler(
    to_dl: web::Form<DlRequest>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, WebErrors> {
    let video_req = VideoRequest {
        url: to_dl.url.clone(),
    };

    let uid = to_dl.id;

    let config = Config::default();
    let hash = argon2::hash_encoded(to_dl.password.as_bytes(), crate::SALT, &config).unwrap();

    // verify password
    let user = get_userid_db(&app_data.db, uid).await?;
    let resp = if hash == user.password {
        let cli = awc::Client::default();
        let client_port = env::var("CLIENT_PORT").expect("Client port not set in .env file");
        let url = format!("{client_port}/videos/new/{uid}");
        let req = cli
            .post(url)
            .send_json(&video_req)
            .await?
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
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
