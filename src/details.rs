use crate::db;
use crate::new::WorkoutForm;
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

    let rows = db::get_training_set(&pool, &workout_date).await;

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera
        .render("details/training_set_detail.tera", &context)
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

    let rows_event = db::rows_events(&pool).await;
    let rows_parts = db::rows_parts(&pool).await;

    let rows = db::get_training_set_by_id(&pool, training_set_id).await;

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);
    context.insert("training_set_detail", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera.render("details/edit.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}




#[post("/training_set/{workout_date}/edit/{training_set_id}")]
async fn update_training_set(
    pool: web::Data<PgPool>,
    form: web::Form<WorkoutForm>,
) -> HttpResponse {
    let workout_form = form.into_inner();

    let new_event_id = if workout_form.event_name.is_empty() == false {
        db::new_workout_event_id(&pool, &workout_form.event_name).await
    } else {
        workout_form.event_id.parse::<i32>().unwrap()
    };

    let new_parts_id = if workout_form.parts_name.is_empty() == false {
        db::new_workout_parts_id(&pool, &workout_form.parts_name).await
    } else {
        workout_form.parts_id.parse::<i32>().unwrap()
    };

    db::update_training_set(
        &pool,
        db::TrainingSetDetail {
            training_set_id: 0,
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
