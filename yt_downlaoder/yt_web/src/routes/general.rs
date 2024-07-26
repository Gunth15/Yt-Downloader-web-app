use crate::handlers::general::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(landing_handler))
            .route("/login", web::get().to(login_handler))
            .route("/{anything_else}", web::get().to(err_handler)),
    );
}
