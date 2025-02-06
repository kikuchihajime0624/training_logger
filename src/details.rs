use crate::db;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use sqlx::PgPool;
use tera::{Context, Tera};
use crate::new::WorkoutForm;

#[get("/training_set/{workout_date}")]
async fn training_set_detail(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    get_training_detail(
        workout_date.into_inner(),
        &tera,
        &pool,
        "details/training_set_detail.tera",
    )
    .await
}

// edit
#[get("/training_set/{workout_date}/edit")]
async fn training_set_edit(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    get_training_detail(
        workout_date.into_inner(),
        &tera,
        &pool,
        "details/edit.tera",
    )
    .await
}

async fn get_training_detail(
    workout_date_get: NaiveDate,
    tera: &Tera,
    pool: &PgPool,
    template: &str,
) -> HttpResponse {

    let rows = db::get_training_set(&pool, &workout_date_get).await;

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date_get);

    let rendered = tera.render(template, &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}


#[post("/training_set/{workout_date}/edit")]
async fn training_set_update(pool: web::Data<PgPool>, form: web::Form<WorkoutForm>) -> HttpResponse {
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

    db::insert_training_set(
        &pool,
        db::NewWorkout {
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