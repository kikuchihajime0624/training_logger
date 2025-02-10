use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub(crate) async fn get_user_by_username(pool: &PgPool, username: String) -> User {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 ")
        .bind(username)
        .fetch_one(pool)
        .await
        .unwrap()
}
