use crate::db;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};

#[derive(Debug, Deserialize)]
struct WorkoutForm {
    //ユーザーがデータベースに入力する値
    event_id: Option<i32>,
    event_name: String,
    parts_id: Option<i32>,
    parts_name: String,
    weight: i32,
    times: i32,
    workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
}

#[derive(Debug, FromRow, Serialize)]
struct TrainingSet {
    //HTMLがデータベースから受け取る値
    training_set_id: i32,
    event_id: i32,
    parts_id: i32,
    weight: i32,
    times: i32,
    workout_date: Option<NaiveDate>,
    // NULLが入るかもしれない時はOptionにする
}



#[get("/new")]
async fn new_log_events(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows_event = db::rows_events(&pool).await;
    let rows_parts = db::rows_parts(&pool).await;

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);

    let rendered = tera.render("new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/new")]
async fn new_training_set(pool: web::Data<PgPool>, form: web::Form<WorkoutForm>) -> HttpResponse {
    let workout_form = form.into_inner();

    let mut new_event = if workout_form.event_name.is_empty() == false {
        ////////////////////////////////////////////////////////////////////////////////

        let new_event_id: i32 = sqlx::query_scalar(
            // db::insert_training_log(&PgPool)

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
        new_event_id

        //////
    } else {
        workout_form.event_id.unwrap()
    };

    let new_parts = if workout_form.parts_name.is_empty() == false {
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

        new_parts_id
    } else {
        workout_form.parts_id.unwrap()
    };

    sqlx::query(
        "INSERT INTO training_set(workout_date, event_id, parts_id,  weight, times)
        VALUES ($1, $2, $3, $4, $5 )",
    )
    .bind(workout_form.workout_date)
    .bind(new_event)
    .bind(new_parts)
    .bind(workout_form.weight)
    .bind(workout_form.times)
    .execute(pool.as_ref())
    .await
    .unwrap();

    ///////////////////////////////////////////////////////////////////////////////////////////

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
