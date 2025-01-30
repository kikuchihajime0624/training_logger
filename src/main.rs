use actix_web::{get, web, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use sqlx::{FromRow, PgPool};
use std::env;
use chrono::NaiveDate;
use tera::{Context, Tera};
use serde::{Deserialize, Serialize};

// #[derive(Template)]
// struct Workouts {
//     date: Date<Local>,
//     evnet_name: String,
//     training_parts: String,
//     weight: i32,
//     times: i32,
// }

#[derive(Debug, Deserialize)]
struct WorkoutForm {
    //ユーザーがデータベースに入力する値
    event_name: String,
    parts_name: String,
    weight: i32,
    times: i32,
    workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
}


#[derive(Debug, FromRow, Serialize)]
struct WorkoutSet {
    //HTMLがデータベースから受け取る値
    training_set_id: i32,
    event_id: i32,
    parts_id: i32,
    weight: i32,
    times: i32,
    workout_date: Option<NaiveDate>,
  // NULLが入るかもしれない時はOptionにする
}



#[get("/")]
async fn dates(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, WorkoutSet>("SELECT workout_date FROM training_set")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("date_list", &rows);

    let rendered = tera.render("training_logger.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// #[post("/new")]
// async fn new(pool: web::Data<PgPool>, form: web::Form<ItemRequest>) -> HttpResponse {
//     let item_request = form.into_inner();
//
//     sqlx::query(
//         "INSERT INTO item (date, event_name, training_parts, weight, times)
//         VALUES ($1, $2, $3, $4, $5)")
//         .bind(item_request.date)
//         .bind(item_request.event_name)
//         .bind(item_request.training_parts)
//         .bind(item_request.weight)
//         .bind(item_request.times)
//         .await
//         .unwrap();
//
//     HttpResponse::Found().append_header(("Location", "/")).finish()
// }

// #[get("/detail")]
// async fn detail(pool: web::Data<Postgres>) -> HttpResponse {
//
// }
//
// #[post("/detail/edit")]
// async fn edit_detail(pool: web::Data<Postgres>) -> HttpResponse {
//
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("環境変数にDATABASE_URLがありません");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("コネクションプール作成エラー");

    let port_string = env::var("PORT").expect("環境変数にPORTがありません");
    let port = port_string.parse::<u16>().expect("環境変数にPORTの形式が不正です");



    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("error in tera/templates");
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .service(dates)
            // .service(new)
            // .service(detail)
            // .service(edit)
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
