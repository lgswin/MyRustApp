use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}