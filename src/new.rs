use crate::db;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};

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
    let rows_event = db::get_events(&pool).await;
    let rows_parts = db::get_parts(&pool).await;

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);

    let rendered = tera.render("new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[derive(Debug, Deserialize)]
pub struct WorkoutForm {
    //ユーザーがデータベースに入力する値
    pub(crate) event_id: String,
    pub(crate) event_name: String,
    pub(crate) parts_id: String,
    pub(crate) parts_name: String,
    pub(crate) weight: i32,
    pub(crate) times: i32,
    pub(crate) workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
}
#[post("/new")]
async fn new_training_set(pool: web::Data<PgPool>, form: web::Form<WorkoutForm>) -> HttpResponse {
    let workout_form = form.into_inner();

    let new_event_id = if workout_form.event_name.is_empty() == false {
        db::register_training_event(&pool, &workout_form.event_name).await
    } else {
        workout_form.event_id.parse::<i32>().unwrap()
    };

    let new_parts_id = if workout_form.parts_name.is_empty() == false {
        db::register_training_parts(&pool, &workout_form.parts_name).await
    } else {
        workout_form.parts_id.parse::<i32>().unwrap()
    };

    db::register_training_set(
        &pool,
        db::NewTrainingSet {
            event_id: new_event_id,
            event_name: workout_form.event_name,
            parts_id: new_parts_id,
            parts_name: workout_form.parts_name,
            weight: workout_form.weight,
            times: workout_form.times,
            workout_date: workout_form.workout_date,
        },
    )
    .await;

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
