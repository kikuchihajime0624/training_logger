use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};

#[derive(Debug, FromRow, Serialize)]
struct TrainingSetDetail {
    //HTMLがデータベースから受け取る値
    event_name: String,
    parts_name: String,
    weight: i32,
    times: i32,
}

#[get("/training_set/{workout_date}")]
async fn detail(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let workout_date = workout_date.into_inner();

    let rows = sqlx::query_as::<_, TrainingSetDetail>(
        "SELECT te.event_name, tp.parts_name, ts.weight, ts.times FROM training_set AS ts
    INNER JOIN training_event AS te ON ts.event_id = te.event_id
    INNER JOIN training_parts AS tp ON ts.parts_id = tp.parts_id
    WHERE ts.workout_date =  $1
    ORDER BY training_set_id",
    )
        .bind(&workout_date)
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera.render("details/training_set_detail.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
