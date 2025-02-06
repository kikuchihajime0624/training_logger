use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;
use crate::db;
use sqlx::PgPool;
use tera::{Context, Tera};

#[get("/training_set/{workout_date}")]
async fn detail(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let workout_date_get = workout_date.into_inner();

    let rows = db::get_training_set(&pool, &workout_date_get).await;

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date_get);

    let rendered = tera
        .render("details/training_set_detail.tera", &context)
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// edit
// #[get("/training_set/{workout_date}/edit")]
// async fn edit_detail(){
//
// }
