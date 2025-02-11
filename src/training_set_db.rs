use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct TrainingSummary {
    pub workout_date: NaiveDate,
    pub total_weight: f64,
}
pub async fn get_training_summary_list(
    pool: &PgPool,
    selected_year: i32,
    selected_month: i32,
    username: String,
) -> Vec<TrainingSummary> {
    sqlx::query_as::<_, TrainingSummary>(
        "SELECT workout_date, SUM(weight * times) AS total_weight
            FROM training_set
            WHERE EXTRACT(YEAR FROM workout_date) = $1 AND EXTRACT(MONTH FROM workout_date) = $2
            AND username = $3
            GROUP BY workout_date
            ORDER BY workout_date DESC
            ",
    )
    .bind(selected_year)
    .bind(selected_month)
    .bind(username)
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn get_oldest_year(pool: &PgPool, username: String) -> Option<NaiveDate> {
    sqlx::query_scalar("SELECT MIN(workout_date) FROM training_set WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[derive(Debug, FromRow, Serialize)]
pub struct TrainingEvent {
    pub event_id: i32,
    pub event_name: String,
    pub username: String,
}

pub async fn get_events(pool: &PgPool, username: String) -> Vec<TrainingEvent> {
    sqlx::query_as::<_, TrainingEvent>(
        "SELECT * FROM training_event WHERE username = $1 ORDER BY event_id",
    )
    .bind(username)
    .fetch_all(pool)
    .await
    .unwrap()
}
#[derive(Debug, FromRow, Serialize)]
pub struct TrainingPart {
    pub parts_id: i32,
    pub parts_name: String,
    pub username: String,
}

pub async fn get_parts(pool: &PgPool, username: String) -> Vec<TrainingPart> {
    sqlx::query_as::<_, TrainingPart>(
        "SELECT * FROM training_parts WHERE username = $1 ORDER BY parts_id",
    )
    .bind(username)
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn register_training_event(pool: &PgPool, event_name: &String, username: String) -> i32 {
    sqlx::query_scalar(
        "
        INSERT INTO training_event(event_name, username)
        VALUES ($1, $2)

        RETURNING event_id
        ",
    )
    .bind(event_name)
    .bind(username)
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn register_training_parts(pool: &PgPool, parts_name: &String, username: String) -> i32 {
    sqlx::query_scalar(
        "
        INSERT INTO training_parts(parts_name, username)
        VALUES ($1, $2)

        RETURNING parts_id
        ",
    )
    .bind(parts_name)
    .bind(username)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[derive(Debug, Deserialize)]
pub struct NewTrainingSet {
    pub event_id: i32,
    pub parts_id: i32,
    pub weight: f32,
    pub times: i32,
    pub workout_date: NaiveDate,
    pub username: String,
}

pub async fn register_training_set(pool: &PgPool, new_training_set: NewTrainingSet) {
    sqlx::query(
        "INSERT INTO training_set(workout_date, event_id, parts_id,  weight, times, username)
        VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&new_training_set.workout_date)
    .bind(&new_training_set.event_id)
    .bind(&new_training_set.parts_id)
    .bind(&new_training_set.weight)
    .bind(&new_training_set.times)
    .bind(&new_training_set.username)
    .execute(pool)
    .await
    .unwrap();
}

#[derive(Debug, FromRow, Serialize)]
pub struct TrainingSetDetail {
    pub training_set_id: i32,
    pub event_name: String,
    pub event_id: i32,
    pub parts_name: String,
    pub parts_id: i32,
    pub weight: f32,
    pub times: i32,
    pub workout_date: NaiveDate,
}
pub async fn get_training_set_by_workout_date(
    pool: &PgPool,
    workout_date: &NaiveDate,
    username: String,
) -> Vec<TrainingSetDetail> {
    sqlx::query_as::<_, TrainingSetDetail>(
        "SELECT ts.training_set_id, te.event_name, te.event_id, tp.parts_name, tp.parts_id, ts.weight, ts.times, ts.workout_date
            FROM training_set AS ts
            INNER JOIN training_event AS te ON ts.event_id = te.event_id
            INNER JOIN training_parts AS tp ON ts.parts_id = tp.parts_id
            WHERE ts.workout_date = $1 AND ts.username = $2
            ORDER BY ts.training_set_id",
    )
        .bind(workout_date)
        .bind(username)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn get_training_set_by_id(
    pool: &PgPool,
    training_set_id: i32,
    username: String,
) -> TrainingSetDetail {
    sqlx::query_as::<_, TrainingSetDetail>(
        "SELECT ts.training_set_id, te.event_name, te.event_id, tp.parts_name, tp.parts_id, ts.weight, ts.times, ts.workout_date
            FROM training_set AS ts
            INNER JOIN training_event AS te ON ts.event_id = te.event_id
            INNER JOIN training_parts AS tp ON ts.parts_id = tp.parts_id
            WHERE ts.training_set_id = $1 AND ts.username = $2
            ORDER BY ts.training_set_id",
    )
        .bind(training_set_id)
        .bind(username)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[derive(Debug, Deserialize)]
pub struct TrainingSet {
    pub training_set_id: i32,
    pub event_id: i32,
    pub parts_id: i32,
    pub weight: f32,
    pub times: i32,
    pub workout_date: NaiveDate,
    pub username: String,
}
pub async fn update_training_set(pool: &PgPool, update_training_set: TrainingSet) {
    sqlx::query(
        "UPDATE training_set
            SET workout_date = $1, event_id = $2, parts_id = $3,  weight = $4, times = $5

            WHERE training_set_id = $6 AND username = $7",
    )
    .bind(&update_training_set.workout_date)
    .bind(&update_training_set.event_id)
    .bind(&update_training_set.parts_id)
    .bind(&update_training_set.weight)
    .bind(&update_training_set.times)
    .bind(&update_training_set.training_set_id)
    .bind(&update_training_set.username)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn delete_training_set(pool: &PgPool, training_set_id: i32, username: String) {
    sqlx::query(
        "DELETE FROM training_set
             WHERE training_set_id = $1 AND username = $2",
    )
    .bind(training_set_id)
    .bind(username)
    .execute(pool)
    .await
    .unwrap();
}
