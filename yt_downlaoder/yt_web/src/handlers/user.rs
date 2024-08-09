use crate::dbscripts::*;
use crate::errors::WebErrors;
use crate::models::{NewUser, UpdateUser};
use crate::state::AppData;
use actix_web::{web, HttpResponse};
use argon2::{self, Config};

//update user creadentials based and validate with existing user info
pub async fn get_update_user(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, WebErrors> {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", &false);
    ctx.insert("fail_msg", "");

    let update = tmpl.render("update.html", &ctx)?;
    Ok(HttpResponse::Ok().body(update))
}
pub async fn put_update_user(
    update_usr: web::Form<UpdateUser>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, WebErrors> {
    let old_usr = get_user_db(&app_data.db, &update_usr.username).await?;

    let config = Config::default();
    let hash =
        argon2::hash_encoded(update_usr.old_password.as_bytes(), crate::SALT, &config).unwrap();

    let resp = if hash == old_usr.password {
        update_user_db(&app_data.db, update_usr.into()).await?
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("fail", &true);
        ctx.insert(
            "fail_msg",
            "current password does not match entered password",
        );

        tmpl.render("update.html", &ctx)?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn get_delete_user(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, WebErrors> {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", &false);
    ctx.insert("fail_msg", "");

    let res = tmpl.render("delete.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(res))
}
pub async fn delete_user(
    delete_usr: web::Form<UpdateUser>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, WebErrors> {
    let old_usr = get_user_db(&app_data.db, &delete_usr.username).await?;

    let config = Config::default();
    let hash =
        argon2::hash_encoded(delete_usr.old_password.as_bytes(), crate::SALT, &config).unwrap();

    let resp = if hash == old_usr.password && delete_usr.new_password == delete_usr.old_password {
        delete_user_db(&app_data.db, &delete_usr.username).await?
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("fail", "true");
        ctx.insert(
            "fail_msg",
            "current password does not match entered password or entered passwords do not match",
        );

        tmpl.render("delete.html", &ctx).unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn get_new_user(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, WebErrors> {
    let resp = tmpl.render("new-user.html", &tera::Context::new())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
pub async fn post_new_user(
    new_user: web::Form<NewUser>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, WebErrors> {
    let resp = post_user_db(&data.db, new_user.into()).await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(resp))
}
