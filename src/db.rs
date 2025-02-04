use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{pool, FromRow, PgPool};

//#[get("/new")]
#[derive(Debug, FromRow, Serialize)]
pub struct TrainingEvent {
    pub event_id: i32,
    pub event_name: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TrainingPart {
    pub parts_id: i32,
    pub parts_name: String,
}
pub async fn rows_events(pool: &PgPool) -> Vec<TrainingEvent> {
    sqlx::query_as::<_, TrainingEvent>("SELECT * FROM training_event ORDER BY event_id")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn rows_parts(pool: &PgPool) -> Vec<TrainingPart> {
    sqlx::query_as::<_, TrainingPart>("SELECT * FROM training_parts ORDER BY parts_id")
        .fetch_all(pool)
        .await
        .unwrap()
}

//#[post("/new")]

#[derive(Debug, Deserialize)]
struct Workout {
    //ユーザーがデータベースに入力する値
    event_id: Option<i32>,
    event_name: String,
    parts_id: Option<i32>,
    parts_name: String,
    weight: i32,
    times: i32,
    workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
}

pub async fn new_workout_event_id(pool: &PgPool, event_name:String) -> i32 {
    sqlx::query_scalar(
        "
        INSERT INTO training_event(event_name)
        VALUES ($1)

        RETURNING event_id
        ",
    )
    .bind(event_name)
    .fetch_one(pool)
    .await
    .unwrap()

}

pub async fn new_workout_parts_id(pool: &PgPool, parts_name:String) -> i32 {
    sqlx::query_scalar(
        "
        INSERT INTO training_parts(parts_name)
        VALUES ($1)

        RETURNING parts_id
        ",
    )
        .bind(parts_name)
        .fetch_one(pool)
        .await
        .unwrap()
}
