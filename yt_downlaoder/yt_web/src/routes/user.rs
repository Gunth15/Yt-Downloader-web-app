use crate::handlers::*;
use actix_web::web;

pub fn update_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("update")
        .route("/{user_id}", web::get().to(get_update_user))
        .route("/{user_id}", web::put().to(put_updated_user))
    );
}
pub fn delete_user_routes(cfg: &mut web::ServiceConfig) {
   cfg.service( 
       web::scope("delete")
        .route("/{user_id}", web::get().to(get_delete_user))
        .route("/{user_id}", web::delete().(delete_user))
    );
}
pub fn new_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("new")
            .route("/", web::get().to(get_new_user))
            .route("/", web::post().to(post_new_user)),
    );
}
