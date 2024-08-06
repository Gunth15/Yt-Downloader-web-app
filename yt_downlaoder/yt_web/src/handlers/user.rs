use crate::dbscripts::*;
use crate::models::{NewUser, UpdateUser};
use crate::state::AppData;
use actix_web::{web, HttpResponse};
use argon2::{password_hash::Salt, Argon2, PasswordHasher};

//update user creadentials based and validate with existing user info
pub async fn get_update_user(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", &false);
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

    let config = Argon2::default();
    let hash = config
        .hash_password(
            update_usr.old_password.as_bytes(),
            Salt::from_b64(crate::SALT).unwrap(),
        )
        .unwrap()
        .to_string();

    let resp = if &hash == &old_usr.password {
        update_user_db(&app_data.db, update_usr.into()).await;
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("fail", &true);
        ctx.insert(
            "fail_msg",
            "current password does not match entered password",
        );

        tmpl.render("update.html", &ctx).unwrap();
    };
    HttpResponse::Ok().content_type("text/html").body(resp)
}
pub async fn get_delete_user(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("fail", &false);
    ctx.insert("fail_msg", "");

    let res = tmpl.render("delete.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(res)
}
pub async fn delete_user(
    delete_usr: web::Form<UpdateUser>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> HttpResponse {
    let old_usr = get_user_db(&app_data.db, &delete_usr.username).await;

    let config = Argon2::default();
    let hash = config
        .hash_password(
            delete_usr.old_password.as_bytes(),
            Salt::from_b64(crate::SALT).unwrap(),
        )
        .unwrap()
        .to_string();
    let resp = if &hash == &old_usr.password {
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
    HttpResponse::Ok().content_type("text/html").body(resp)
}
pub async fn get_new_user(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let resp = tmpl.render("new-user.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}
pub async fn post_new_user(new_user: web::Form<NewUser>, data: web::Data<AppData>) -> HttpResponse {
    let resp = post_user_db(&data.db, new_user.into()).await;
    HttpResponse::Ok().content_type("text/html").body(resp)
}
