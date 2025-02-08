mod db;
mod details;
mod new;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use chrono::NaiveDate;
use dotenvy::dotenv;

use sqlx::PgPool;
use std::env;
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_scalar::<_, NaiveDate>(
        "SELECT DISTINCT workout_date FROM training_set ORDER BY workout_date DESC",
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap();

    let mut context = Context::new();
    context.insert("workout_date_list", &rows);

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
