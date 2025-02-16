use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use my_rust_app::{health_check, create_user, get_user, AppState};  // Import from lib.rs
use sqlx::MySqlPool;
use std::{collections::HashMap, env, sync::Mutex};
use dotenv::dotenv;
use simplelog::*;
use std::fs::File;

use std::path::Path;

async fn index() -> impl Responder {
    let path = Path::new("templates/index.html");
    if path.exists() {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(std::fs::read_to_string(path).unwrap_or_else(|_| "<h1>Error: Could not load file</h1>".to_string()))
    } else {
        HttpResponse::InternalServerError().body("<h1>File not found in container--</h1>")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file

    // Create the log directory if it doesn't exist
    std::fs::create_dir_all("/var/log/myapp").expect("Failed to create log directory");

    // Create a log file where log messages will be written
    let log_file = File::create("/var/log/myapp/application.log").unwrap();

    // Initialize the logger to write logs at the Info level (and above) to the file
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ]).unwrap();

    // Log an informational message indicating that the application has started
    log::info!("Application started");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to MySQL");

    let state = web::Data::new(AppState { 
        pool,
        users: Mutex::new(HashMap::new()),  // Initialize users field
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))  // ✅ 루트 경로 추가
            .route("/health", web::get().to(health_check))
            .route("/user", web::post().to(create_user))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}