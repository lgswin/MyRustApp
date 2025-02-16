#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, http::StatusCode};
    use serde_json::json;
    use my_rust_app::{create_user, get_user, AppState, User};  // Use `my_rust_app` instead of `crate::`
    use std::sync::Mutex;
    use std::collections::HashMap;
    use sqlx::MySqlPool;
    

    // ✅ Function to create a test database connection
    async fn get_test_pool() -> MySqlPool {
        let database_url = "mysql://rust_user:rust_password@localhost:3306/rust_db"; // Change to a test DB
        MySqlPool::connect(database_url).await.expect("Failed to connect to test database")
    }

    #[actix_rt::test]
    async fn test_create_user() {
        let pool = get_test_pool().await;  // ✅ Initialize database pool
        let state = web::Data::new(AppState {
            pool,
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/user", web::post().to(create_user)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&json!({ "id": 1, "firstname": "Alice", "lastname": "Lee" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_get_user() {
        let pool = get_test_pool().await;  // Initialize database pool
        let state = web::Data::new(AppState {
            pool,
            users: Mutex::new(HashMap::new()),
        });

        state.users.lock().unwrap().insert(
            1,
            User {
                id: 1,
                firstname: "Alice".to_string(),
                lastname: "Lee".to_string(),
            },
        );

        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/user/{id}", web::get().to(get_user)),
        )
        .await;

        let req = test::TestRequest::get().uri("/user/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}