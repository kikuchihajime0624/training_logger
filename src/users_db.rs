use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub async fn get_user_by_username(pool: &PgPool, username: String) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 ")
        .bind(username)
        .fetch_one(pool)
        .await
        .ok()
}

pub async fn register_user(pool: &PgPool, user: User) {
    sqlx::query(
        "  INSERT INTO users(username, password)
        VALUES ($1, $2)
        ",
    )
        .bind(user.username)
        .bind(user.password)
        .execute(pool)
        .await
        .unwrap();
}
