use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;

// Shared user storage (in-memory)
struct AppState {
    users: Mutex<HashMap<u32, User>>,
}

// User struct
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Helper function to log data
fn log_to_file(data: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("logs.txt")
        .unwrap();
    writeln!(file, "{}", data).unwrap();
}

// Create user (POST /user)
async fn create_user(data: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    users.insert(data.id, data.clone());

    let log_entry = format!("Created user: {:?}", data);
    log_to_file(&log_entry);

    HttpResponse::Created().json(data.into_inner())
}

// Get user by ID (GET /user/{id})
async fn get_user(id: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();

    if let Some(user) = users.get(&id.into_inner()) {
        let log_entry = format!("Fetched user: {:?}", user);
        log_to_file(&log_entry);
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/user", web::post().to(create_user))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}