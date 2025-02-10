use crate::training_set_db;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};

#[derive(Debug, FromRow, Serialize)]
struct TrainingSet {
    training_set_id: i32,
    event_id: i32,
    parts_id: i32,
    weight: f32,
    times: i32,
    workout_date: NaiveDate,
}

#[get("/new")]
async fn get_new_training_set(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows_event = training_set_db::get_events(&pool).await;
    let rows_parts = training_set_db::get_parts(&pool).await;

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);

    let rendered = tera.render("new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[derive(Debug, Deserialize)]
pub struct TrainingSetForm {
    //ユーザーがデータベースに入力する値
    pub(crate) event_id: Option<i32>,
    pub(crate) event_name: Option<String>,
    pub(crate) parts_id: Option<i32>,
    pub(crate) parts_name: Option<String>,
    pub(crate) weight: f32,
    pub(crate) times: i32,
    pub(crate) workout_date: NaiveDate,
}
#[post("/new")]
async fn post_new_training_set(
    pool: web::Data<PgPool>,
    form: web::Form<TrainingSetForm>,
) -> HttpResponse {
    let training_set_form = form.into_inner();

    let new_event_id = if training_set_form.event_name.is_some()  {
        training_set_db::register_training_event(&pool, &training_set_form.event_name.unwrap()).await
    } else {
        training_set_form.event_id.unwrap()
    };

    let new_parts_id = if training_set_form.parts_name.is_some() {
        training_set_db::register_training_parts(&pool, &training_set_form.parts_name.unwrap()).await
    } else {
        training_set_form.parts_id.unwrap()
    };

    training_set_db::register_training_set(
        &pool,
        training_set_db::NewTrainingSet {
            event_id: new_event_id,
            parts_id: new_parts_id,
            weight: training_set_form.weight,
            times: training_set_form.times,
            workout_date: training_set_form.workout_date,
        },
    )
        .await;

    HttpResponse::Found()
        .append_header((
            "Location",
            format!("/training_set/{}", training_set_form.workout_date),
        ))
        .finish()
}


