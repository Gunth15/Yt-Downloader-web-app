use actix_web::web;
use argon2::{self, Config};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
impl From<web::Form<NewUser>> for NewUser {
    fn from(form: web::Form<NewUser>) -> Self {
        let config = Config::default();
        let hash = argon2::hash_encoded(form.password.as_bytes(), crate::SALT, &config).unwrap();
        NewUser {
            username: form.username.clone(),
            password: hash.clone(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub new_password: String,
    pub old_password: String,
}
impl From<web::Form<UpdateUser>> for UpdateUser {
    fn from(form: web::Form<UpdateUser>) -> Self {
        let config = Config::default();
        let hash =
            argon2::hash_encoded(form.new_password.as_bytes(), crate::SALT, &config).unwrap();
        UpdateUser {
            username: form.username.clone(),
            old_password: form.old_password.clone(),
            new_password: hash.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoQuery {
    pub title: String,
    pub url: String,
    pub thumbnail_url: String,
    pub query_time: Option<NaiveDateTime>,
    pub user_id: i32,
    pub video_id: String,
    pub size: i64,
}
impl From<web::Json<VideoQuery>> for VideoQuery {
    fn from(video_query: web::Json<VideoQuery>) -> Self {
        VideoQuery {
            title: video_query.title.clone(),
            url: video_query.url.clone(),
            thumbnail_url: video_query.thumbnail_url.clone(),
            query_time: video_query.query_time,
            user_id: video_query.user_id,
            video_id: video_query.video_id.clone(),
            size: video_query.size,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoRequest {
    pub url: String,
}
impl From<web::Json<VideoRequest>> for VideoRequest {
    fn from(req: web::Json<VideoRequest>) -> Self {
        VideoRequest {
            url: req.url.clone(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct DlRequest {
    pub url: String,
    pub password: String,
    pub id: i32,
}
