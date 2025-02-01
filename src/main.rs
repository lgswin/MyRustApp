use axum::{routing::get, Router, extract::Path, Json};
use serde::Serialize;
use tokio;
use std::net::SocketAddr;

mod lib;
use lib::{greet, Greeting, root_handler};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler)) // Default route
        .route("/greet/:name", get(greet_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Greeting handler for "/greet/:name"
async fn greet_handler(Path(name): Path<String>) -> Json<Greeting> {
    Json(greet(&name))
}