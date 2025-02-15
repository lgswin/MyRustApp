use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
}

pub async fn create_user(
    pool: web::Data<MySqlPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (first_name, last_name) VALUES (?, ?)",
        user.first_name,
        user.last_name
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

pub async fn get_user(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as!(
        User,
        "SELECT id, first_name, last_name FROM users WHERE id = ?",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}