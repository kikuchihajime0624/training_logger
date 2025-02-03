use actix_web::{get, post, web, HttpResponse};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};
use crate::WorkoutForm;

#[derive(Debug, FromRow, Serialize)]
struct NewLogEvent {
    event_id: i32,
    event_name: String,
}

#[derive(Debug, FromRow, Serialize)]
struct NewLogParts{
    parts_id: i32,
    parts_name: String,
}

#[get("/new")]
async fn new_log_events(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows_event =
        sqlx::query_as::<_, NewLogEvent>("SELECT * FROM training_event ORDER BY event_id")
            .fetch_all(pool.as_ref())
            .await
            .unwrap();

    let rows_parts =
        sqlx::query_as::<_, NewLogParts>("SELECT * FROM training_parts ORDER BY parts_id")
            .fetch_all(pool.as_ref())
            .await
            .unwrap();

    let mut context = Context::new();
    context.insert("event_list", &rows_event);
    context.insert("parts_list", &rows_parts);

    let rendered = tera.render("new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/new")]
async fn new_training_set(pool: web::Data<PgPool>, form: web::Form<WorkoutForm>) -> HttpResponse {
    let workout_form = form.into_inner();

    let new_event_id: i32 = sqlx::query_scalar(
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

    sqlx::query(
        "INSERT INTO training_set(workout_date, event_id, parts_id,  weight, times)
        VALUES ($1, $2, $3, $4, $5 )",
    )
        .bind(workout_form.workout_date)
        .bind(new_event_id)
        .bind(new_parts_id)
        .bind(workout_form.weight)
        .bind(workout_form.times)
        .execute(pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
