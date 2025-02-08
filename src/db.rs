use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

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

pub async fn new_workout_event_id(pool: &PgPool, event_name: &String) -> i32 {
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

pub async fn new_workout_parts_id(pool: &PgPool, parts_name: &String) -> i32 {
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

#[derive(Debug, Deserialize)]
pub struct NewTrainingSet {
    pub event_id: i32,
    pub event_name: String,
    pub parts_id: i32,
    pub parts_name: String,
    pub weight: i32,
    pub times: i32,
    pub workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
}




pub async fn insert_training_set(pool: &PgPool, new_workout: NewTrainingSet) {
    sqlx::query(
        "INSERT INTO training_set(workout_date, event_id, parts_id,  weight, times)
        VALUES ($1, $2, $3, $4, $5 )",
    )
    .bind(&new_workout.workout_date)
    .bind(&new_workout.event_id)
    .bind(&new_workout.parts_id)
    .bind(&new_workout.weight)
    .bind(&new_workout.times)
    .execute(pool)
    .await
    .unwrap();
}

//detailのSQL文

#[derive(Debug, FromRow, Serialize)]
pub struct TrainingSetDetail {
    //HTMLがデータベースから受け取る値
    pub training_set_id: i32,
    pub event_name: String,
    pub event_id: i32,
    pub parts_name: String,
    pub parts_id: i32,
    pub weight: i32,
    pub times: i32,
    pub workout_date: Option<NaiveDate>,
}
pub async fn get_training_set(pool: &PgPool, workout_date: &NaiveDate) -> Vec<TrainingSetDetail> {
    sqlx::query_as::<_, TrainingSetDetail>(
        "SELECT ts.training_set_id, te.event_name, te.event_id, tp.parts_name, tp.parts_id, ts.weight, ts.times
            FROM training_set AS ts
            INNER JOIN training_event AS te ON ts.event_id = te.event_id
            INNER JOIN training_parts AS tp ON ts.parts_id = tp.parts_id
            WHERE ts.workout_date = $1
            ORDER BY ts.training_set_id",
    )
    .bind(workout_date)
    .fetch_all(pool)
    .await
    .unwrap()
}

//edit
pub async fn get_training_set_by_id(pool: &PgPool, training_set_id: i32) -> TrainingSetDetail {
    sqlx::query_as::<_, TrainingSetDetail>(
        "SELECT ts.training_set_id, te.event_name, te.event_id, tp.parts_name, tp.parts_id, ts.weight, ts.times, ts.workout_date
            FROM training_set AS ts
            INNER JOIN training_event AS te ON ts.event_id = te.event_id
            INNER JOIN training_parts AS tp ON ts.parts_id = tp.parts_id
            WHERE ts.training_set_id = $1
            ORDER BY ts.training_set_id",
    )
        .bind(training_set_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

pub async fn update_events(pool: &PgPool) -> Vec<TrainingEvent> {
    sqlx::query_as::<_, TrainingEvent>("SELECT * FROM training_event ORDER BY event_id")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn update_parts(pool: &PgPool) -> Vec<TrainingPart> {
    sqlx::query_as::<_, TrainingPart>("SELECT * FROM training_parts ORDER BY parts_id")
        .fetch_all(pool)
        .await
        .unwrap()
}
pub async fn update_training_set(pool: &PgPool, update_workout: TrainingSetDetail) {
    sqlx::query(
        "UPDATE training_set
            SET workout_date = $1, event_id = $2, parts_id = $3,  weight = $4, times = $5

            WHERE ts.training_set_id = $6",
    )
    .bind(&update_workout.workout_date)
    .bind(&update_workout.event_id)
    .bind(&update_workout.parts_id)
    .bind(&update_workout.weight)
    .bind(&update_workout.times)
    .bind(&update_workout.training_set_id)
    .execute(pool)
    .await
    .unwrap();
}
