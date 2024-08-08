use crate::handlers::user::*;
use actix_web::web;

pub fn update_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/update")
            .route("", web::get().to(get_update_user))
            .route("", web::post().to(put_update_user)),
    );
}
pub fn delete_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/delete")
            .route("", web::get().to(get_delete_user))
            .route("", web::post().to(delete_user)),
    );
}
pub fn new_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/new")
            .route("", web::get().to(get_new_user))
            .route("", web::post().to(post_new_user)),
    );
}
