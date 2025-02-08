use crate::db;
use crate::new::TrainingSetForm;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use sqlx::PgPool;
use tera::{Context, Tera};

#[get("/training_set/{workout_date}")]
async fn training_set_detail(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let workout_date = workout_date.into_inner();

    let rows = db::get_training_set_by_workout_date(&pool, &workout_date).await;

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera
        .render("detail.tera", &context)
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// edit
#[get("/training_set/{workout_date}/edit/{training_set_id}")]
async fn training_set_edit(
    path: web::Path<(NaiveDate, i32)>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let (workout_date, training_set_id) = path.into_inner();

    let rows_event = db::get_events(&pool).await;
    let rows_parts = db::get_parts(&pool).await;

    let rows = db::get_training_set_by_id(&pool, training_set_id).await;

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);
    context.insert("training_set_detail", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera.render("edit.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/training_set/{workout_date}/edit/{training_set_id}")]
async fn update_training_set(
    pool: web::Data<PgPool>,
    form: web::Form<TrainingSetForm>,
    path: web::Path<(NaiveDate, i32)>,
) -> HttpResponse {
    let training_set_form = form.into_inner();
    let (workout_date, training_set_id) = path.into_inner();

    let new_event_id = if training_set_form.event_name.is_empty() == false {
        db::register_training_event(&pool, &training_set_form.event_name).await
    } else {
        training_set_form.event_id.parse::<i32>().unwrap()
    };

    let new_parts_id = if training_set_form.parts_name.is_empty() == false {
        db::register_training_parts(&pool, &training_set_form.parts_name).await
    } else {
        training_set_form.parts_id.parse::<i32>().unwrap()
    };

    db::update_training_set(
        &pool,
        db::TrainingSet {
            training_set_id,
            event_id: new_event_id,
            parts_id: new_parts_id,
            weight: training_set_form.weight,
            times: training_set_form.times,
            workout_date: training_set_form.workout_date,
        },
    )
        .await;

    HttpResponse::Found()
        .append_header(("Location", format!("/training_set/{}", workout_date)))
        .finish()
}

#[post("/training_set/{workout_date}/delete/{training_set_id}")]
async fn delete_training_set(
    pool: web::Data<PgPool>,
    path: web::Path<(NaiveDate, i32)>,
) -> HttpResponse {
    let (workout_date, training_set_id) = path.into_inner();

    db::delete_training_set(&pool, training_set_id).await;

    HttpResponse::Found()
        .append_header(("Location", format!("/training_set/{}", workout_date)))
        .finish()
}
