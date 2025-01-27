use actix_web::{get, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{PgPool, Postgres};

#[derive(Template)]
#[template(path = "training_logger.html")]
struct LoggerTemplate {}
#[get("/")]
async fn dates(pool: web::Data<Postgres>) -> HttpResponse {
    let dates = LoggerTemplate {};
    dates.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres::memory:").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .service(dates)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
