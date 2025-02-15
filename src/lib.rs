use actix_web::{web, HttpResponse, Responder};  // Import HttpResponse
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub struct AppState {
    pub users: Mutex<HashMap<u32, User>>,
}

// API handlers
pub async fn create_user(data: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    users.insert(data.id, data.into_inner());

    HttpResponse::Created().json(json!({ "message": "User created" }))  // ✅ 201 Created 반환
}

pub async fn get_user(user_id: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    
    if let Some(user) = users.get(&user_id) {
        return HttpResponse::Ok().json(user);  // ✅ Return JSON response
    }

    HttpResponse::NotFound().json(json!({ "error": "User not found" }))  // ✅ Return proper error response
}