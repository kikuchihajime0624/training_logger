use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use chrono::{DateTime, Local};
use sqlx::{PgPool, Postgres};
use tera::Tera;

#[derive(Template)]
struct Workouts {
    date: Date<Local>,
    evnet_name: String,
    training_parts: String,
    weight: i32,
    times: i32,
}
#[get("/")]
async fn dates(pool: web::Data<Postgres>) -> HttpResponse {
    let dates = Workouts {};
    dates.to_response()
}

#[post("/new")]
async fn new(pool: web::Data<PgPool>, form: web::Form<ItemRequest>) -> HttpResponse {
    let item_request = form.into_inner();

    sqlx::query(
        "INSERT INTO item (date, event_name, training_parts, weight, times)
        VALUES ($1, $2, $3, $4, $5)")
        .bind(item_request.date)
        .bind(item_request.event_name)
        .bind(item_request.training_parts)
        .bind(item_request.weight)
        .bind(item_request.times)
        .await
        .unwrap();

    HttpResponse::Found().append_header(("Location", "/")).finish()
}

#[get("/detail")]
async fn detail(pool: web::Data<Postgres>) -> HttpResponse {

}

#[post("/detail/edit")]
async fn edit_detail(pool: web::Data<Postgres>) -> HttpResponse {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres::memory:").await.unwrap();
    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("error in tera/templates");
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .service(dates)
            .service(new)
            .service(detail)
            .service(edit)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
