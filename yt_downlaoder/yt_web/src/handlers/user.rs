use crate::dbscripts::*;
use crate::models::UpdateUser;
use crate::state::AppData;
use actix_web::{web, HttpResponse};

//update user creadentials based and validate with existing user info
pub fn get_update_user(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", "false");
    ctx.insert("fail_msg", "");

    let update = tmpl.render("update.html", &ctx).unwrap();
    HttpResponse::Ok().body(update)
}
pub async fn put_update_user(
    update_usr: web::Form<UpdateUser>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> HttpResponse {
    let old_usr = get_user_db(&app_data.db, &update_usr.username).await;
    let resp = if &update_usr.old_password == &old_usr.password {
        update_user_db(&app_data.db, update_usr.into()).await;
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("fail", "true");
        ctx.insert(
            "fail_msg",
            "current password does not match entered password",
        );

        tmpl.render("update.html", &ctx).unwrap();
    };
    HttpResponse::Ok().body(resp)
}
pub async fn get_delete_user(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", "false");
    ctx.insert("fail_msg", "");

    let res = tmpl.render("delete.html", &ctx).unwrap();
    HttpResponse::Ok().body(res)
}
pub async fn delete_user(
    delete_usr: web::Form<UpdateUser>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> HttpResponse {
    let old_usr = get_user_db(&app_data.db, &delete_usr.username).await;
    let resp = if &delete_usr.old_password == &old_usr.password {
        delete_user_db(&app_data.db, &delete_usr.username).await;
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("fail", "true");
        ctx.insert(
            "fail_msg",
            "current password does not match entered password",
        );

        tmpl.render("delete.html", &ctx).unwrap();
    };
    HttpResponse::Ok().body(resp)
}
pub async fn get_new_user() -> HttpResponse {
    HttpResponse::Ok()
}
pub async fn post_new_user() -> HttpResponse {
    HttpResponse::Ok()
}
