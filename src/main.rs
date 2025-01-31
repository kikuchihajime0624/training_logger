use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use chrono::NaiveDate;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::env;
use tera::{Context, Tera};

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
    let rows = sqlx::query_as::<_, WorkoutSet>("SELECT * FROM training_set ORDER BY workout_date DESC")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("workout_list", &rows);

    let rendered = tera.render("training_logger.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/new")]
async fn new_log_page(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut context = Context::new();

    let rendered = tera.render("new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/new")]
async fn new_training_set(pool: web::Data<PgPool>, form: web::Form<WorkoutForm>) -> HttpResponse {
    let workout_form = form.into_inner();

    let new_event_id: i32 = sqlx::query_scalar(
        "
        INSERT INTO training_event(event_name)
        VALUES ($1)

        RETURNING event_id
        ",
    )
    .bind(workout_form.event_name)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    let new_parts_id: i32 = sqlx::query_scalar(
        "
        INSERT INTO training_parts(parts_name)
        VALUES ($1)

        RETURNING parts_id
        ",
    )
    .bind(workout_form.parts_name)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO training_set(workout_date, event_id, parts_id,  weight, times)
        VALUES ($1, $2, $3, $4, $5 )",
    )
    .bind(workout_form.workout_date)
    .bind(new_event_id)
    .bind(new_parts_id)
    .bind(workout_form.weight)
    .bind(workout_form.times)
    .execute(pool.as_ref())
    .await
    .unwrap();

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

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
    let port = port_string
        .parse::<u16>()
        .expect("環境変数にPORTの形式が不正です");

    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("error in tera/templates");
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .service(dates)
            .service(new_training_set)
            .service(new_log_page)
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
