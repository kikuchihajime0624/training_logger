use crate::users_db;
use actix_web::{get, post, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use sqlx::PgPool;
use tera::{Context, Tera};

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
async fn get_login(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();

    let rendered = tera.render("login.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
#[post("/login")]
async fn post_login(
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
    form: web::Form<LoginForm>,
) -> HttpResponse {
    let form = form.into_inner();

    let user = users_db::get_user_by_username(&pool, form.username).await;

    let mut context = Context::new();
    context.insert("message", "ユーザーIDまたはパスワードが違います");

    let rendered = tera.render("login.tera", &context).unwrap();

    if user.is_none() {
        return HttpResponse::Ok().content_type("text/html").body(rendered);
    }

    if verify(&form.password, &user.unwrap().password).unwrap() == false {
        return HttpResponse::Ok().content_type("text/html").body(rendered);
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[derive(Debug, Deserialize)]
struct SignupForm {
    username: String,
    password: String,
    confirm_password: String,
}
#[get("/signup")]
async fn get_signup(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("username", "");

    let rendered = tera.render("signup.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/signup")]
async fn post_signup(
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
    form: web::Form<SignupForm>,
) -> HttpResponse {
    let form = form.into_inner();

    let mut context = Context::new();
    context.insert("username", &form.username);

    if form.password != form.confirm_password {
        context.insert("message", "パスワードが一致しません");

        let rendered = tera.render("signup.tera", &context).unwrap();
        return HttpResponse::Ok().content_type("text/html").body(rendered);
    }

    let user = users_db::get_user_by_username(&pool, form.username.clone()).await;
    if user.is_some() {
        context.insert("message", "そのユーザーは既に存在します");

        let rendered = tera.render("signup.tera", &context).unwrap();
        return HttpResponse::Ok().content_type("text/html").body(rendered);
    }

    let password = hash(&form.password, DEFAULT_COST).unwrap();

    users_db::register_user(
        &pool,
        users_db::User {
            username: form.username.clone(),
            password,
        },
    )
    .await;

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
