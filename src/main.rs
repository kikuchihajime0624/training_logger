mod db;
mod details;
mod new;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use chrono::{Datelike, Local};
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::PgPool;
use std::env;
use tera::{Context, Tera};

#[derive(Debug, Deserialize)]
struct SelectYearMonth {
    selected_year: Option<i32>,
    selected_month: Option<u32>,
}

#[get("/")]
async fn index(
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
    query: web::Query<SelectYearMonth>,
) -> HttpResponse {


    let current_year = Local::now().year();

    let selected_year = query.selected_year.unwrap_or(Local::now().year());
    let selected_month = query.selected_month.unwrap_or(Local::now().month()) as i32;

    let rows = db::get_training_summary_list(&pool, selected_year,  selected_month).await;

    let oldest_year = db::get_oldest_year(&pool)
        .await
        .map(|workout_date| workout_date.year()).unwrap_or(current_year);

    let mut context = Context::new();
    context.insert("training_summary_list", &rows);
    context.insert("selected_year", &selected_year);
    context.insert("selected_month", &selected_month);
    context.insert("oldest_year", &oldest_year);
    context.insert("current_year", &current_year);

    let rendered = tera.render("index.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("環境変数にDATABASE_URLがありません");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("コネクションプール作成エラー");

    let port_string = env::var("PORT").expect("環境変数にPORTがありません");
    let port = port_string
        .parse::<u16>()
        .expect("環境変数にPORTの形式が不正です");

    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("error in tera/templates");
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .service(index)
            .service(new::new_training_set)
            .service(new::new_log_events)
            .service(details::training_set_detail)
            .service(details::training_set_edit)
            .service(details::update_training_set)
            .service(details::delete_training_set)
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
