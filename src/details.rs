use crate::new::TrainingSetForm;
use crate::training_set_db;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use sqlx::PgPool;
use tera::{Context, Tera};

#[get("/training_set/{workout_date}")]
async fn training_set_detail(
    workout_date: web::Path<NaiveDate>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
    user: Option<Identity>,
) -> HttpResponse {
    let workout_date = workout_date.into_inner();

    let username = user.unwrap().id().unwrap();

    let rows =
        training_set_db::get_training_set_by_workout_date(&pool, &workout_date, username).await;

    let mut context = Context::new();
    context.insert("training_set_detail_list", &rows);
    context.insert("workout_date", &workout_date);

    let rendered = tera.render("detail.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// edit
#[get("/training_set/{workout_date}/edit/{training_set_id}")]
async fn training_set_edit(
    path: web::Path<(NaiveDate, i32)>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
    user: Option<Identity>,
) -> HttpResponse {
    let (workout_date, training_set_id) = path.into_inner();
    let username = user.unwrap().id().unwrap();

    let rows_event = training_set_db::get_events(&pool, username.clone()).await;
    let rows_parts = training_set_db::get_parts(&pool, username.clone()).await;

    let rows =
        training_set_db::get_training_set_by_id(&pool, training_set_id, username.clone()).await;

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
    user: Option<Identity>,
) -> HttpResponse {
    let training_set_form = form.into_inner();
    let (workout_date, training_set_id) = path.into_inner();
    let username = user.unwrap().id().unwrap();

    let new_event_id = if training_set_form.event_name.is_some() {
        training_set_db::register_training_event(
            &pool,
            &training_set_form.event_name.unwrap(),
            username.clone(),
        )
        .await
    } else {
        training_set_form.event_id.unwrap()
    };

    let new_parts_id = if training_set_form.parts_name.is_some() {
        training_set_db::register_training_parts(&pool, &training_set_form.parts_name.unwrap(), username.clone())
            .await
    } else {
        training_set_form.parts_id.unwrap()
    };

    training_set_db::update_training_set(
        &pool,
        training_set_db::TrainingSet {
            training_set_id,
            event_id: new_event_id,
            parts_id: new_parts_id,
            weight: training_set_form.weight,
            times: training_set_form.times,
            workout_date: training_set_form.workout_date,
            username: username.clone()
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
    user: Option<Identity>,
) -> HttpResponse {
    let (workout_date, training_set_id) = path.into_inner();
    let username = user.unwrap().id().unwrap();

    training_set_db::delete_training_set(&pool, training_set_id, username.clone()).await;

    HttpResponse::Found()
        .append_header(("Location", format!("/training_set/{}", workout_date)))
        .finish()
}
