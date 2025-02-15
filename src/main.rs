use actix_web::{web, App, HttpServer};
use my_rust_app::{health_check, create_user, get_user, AppState};  // Import from lib.rs
use sqlx::{MySql, MySqlPool};
use std::{collections::HashMap, env, sync::Mutex};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to MySQL");

    let state = web::Data::new(AppState { 
        pool,
        users: Mutex::new(HashMap::new()),  // ✅ Initialize users field
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/health", web::get().to(health_check))
            .route("/user", web::post().to(create_user))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}