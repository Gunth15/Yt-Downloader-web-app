use actix_web::{web, HttpResponse};
use tera::Tera;

//display landing page
pub async fn landing_handler(tmpl: web::Data<Tera>) -> HttpResponse {
    let resp = tmpl.render("landing.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}

//disply login screen
pub async fn login_handler(tmpl: web::Data<Tera>) -> HttpResponse {
    let resp = tmpl.render("login.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}

//error 404
pub async fn err_handler(tmpl: web::Data<Tera>) -> HttpResponse {
    let resp = tmpl.render("404.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}
