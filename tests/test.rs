#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, http::StatusCode};
    use serde_json::json;
    use crate::{create_user, get_user, AppState, User};
    use std::sync::Mutex;
    use std::collections::HashMap;

    #[actix_rt::test]
    async fn test_create_user() {
        let state = web::Data::new(AppState {
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
            .set_json(&json!({ "id": 1, "name": "Alice", "email": "alice@example.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_get_user() {
        let state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        // Insert test user
        state.users.lock().unwrap().insert(
            1,
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
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