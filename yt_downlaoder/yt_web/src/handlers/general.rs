use actix_web::{web, HttpResponse};
use tera::Tera;

use crate::{dbscripts::get_user_db, errors::WebErrors, models::NewUser, state::AppData};

//display landing page
pub async fn landing_handler(tmpl: web::Data<Tera>) -> Result<HttpResponse, WebErrors> {
    let mut ctx = tera::Context::new();
    ctx.insert("logged_in", &false);
    ctx.insert("Error", "");

    let resp = tmpl.render("landing.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}

//disply login screen
pub async fn login_handler(tmpl: web::Data<Tera>) -> Result<HttpResponse, WebErrors> {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", &false);
    ctx.insert("fail_msg", "");

    let resp = tmpl.render("login.html", &tera::Context::new())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn verify_login_handler(
    tmpl: web::Data<Tera>,
    data: web::Data<AppData>,
    form: web::Form<NewUser>,
) -> Result<HttpResponse, WebErrors> {
    let mut ctx = tera::Context::new();
    let user = get_user_db(&data.db, &form.username).await?;

    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(form.password.as_bytes(), crate::SALT, &config).unwrap();

    //if successful, print landing with logged in screen
    let resp = if hash == user.password {
        ctx.insert("logged_in", &true);
        ctx.insert("Error", "");
        ctx.insert("id", &user.user_id);
        tmpl.render("landing.html", &ctx)?
    //Othewhise return failure message with lgin screen
    } else {
        ctx.insert("fail", &true);
        ctx.insert("fail_msg", "Incorrect password or username");
        tmpl.render("login.html", &ctx)?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}

//error 404
pub async fn err_handler(tmpl: web::Data<Tera>) -> HttpResponse {
    let resp = tmpl.render("404.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}
