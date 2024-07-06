pub mod routes {}
pub mod handlers {
    use actix_web::{web, HttpResponse};
    use yt_service::models::VideoRequest;

    pub fn get_details(path: web::Path<String>) -> HttpResponse {
        let url = path.into_inner();
    }
    pub fn download_video() {}
    pub fn delete_video() {}
    pub fn delete_all() {}
}
