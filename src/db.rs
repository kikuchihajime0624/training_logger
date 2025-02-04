use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{pool, FromRow, PgPool};


//#[get("/new")]
#[derive(Debug, FromRow, Serialize)]
pub struct NewLogEvent {
    pub event_id: i32,
    pub event_name: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct NewLogParts {
    pub parts_id: i32,
    pub parts_name: String,
}
pub async fn rows_events(pool: &PgPool) -> Vec<NewLogEvent> {
    sqlx::query_as::<_, NewLogEvent>("SELECT * FROM training_event ORDER BY event_id")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn rows_parts(pool: &PgPool) -> Vec<NewLogParts> {
    sqlx::query_as::<_, NewLogParts>("SELECT * FROM training_parts ORDER BY parts_id")
        .fetch_all(pool)
        .await
        .unwrap()
}


//#[post("/new")]
// #[derive(Debug, Deserialize)]
// struct Workout{
//     //ユーザーがデータベースに入力する値
//     event_id: Option<i32>,
//     event_name: String,
//     parts_id: Option<i32>,
//     parts_name: String,
//     weight: i32,
//     times: i32,
//     workout_date: Option<NaiveDate>, // NULLが入るかもしれない時はOptionにする
// }

