use crate::state::AppData;
use actix_web::{web, HttpResponse};
use tera::Tera;

//display landing page
pub fn landing_handler(tmpl: web::Data<Tera>) -> HttpResponse {}

//disply login screen
pub fn login_handler(tera: web::Data<Tera>) -> HttpResponse {}

//error 404
pub fn err_handler(tera: web::Data<Tera>) -> HttpResponse {}
