use actix_web::{web, App, HttpServer};
use my_rust_app::{health_check, create_user, get_user, AppState};  // Import from lib.rs
use std::sync::Mutex;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/health", web::get().to(health_check))
            .route("/user", web::post().to(create_user))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}