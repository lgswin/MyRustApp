#[cfg(test)]
mod tests {
    use my_rust_app::{greet, root_handler};
    use axum::{Router, routing::get};
    use tower::ServiceExt; // For `oneshot()`
    use hyper::{Request, Body, StatusCode};

    #[tokio::test]
    async fn test_greet() {
        let greeting = greet("Alice");
        assert_eq!(greeting.message, "Hello, Alice!");
    }

    #[tokio::test]
    async fn test_greet_empty() {
        let greeting = greet("");
        assert_eq!(greeting.message, "Hello, !");
    }

    #[tokio::test]
    async fn test_greet_special_chars() {
        let greeting = greet("@User123");
        assert_eq!(greeting.message, "Hello, @User123!");
    }

    #[tokio::test]
    async fn test_default_route() {
        // Create an instance of the app
        let app = Router::new().route("/", get(root_handler));

        // Create a request to "/"
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Check that the response status is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Convert the response body to a string
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();

        // Ensure the response body is "Hello, Rust."
        assert_eq!(body_str, "Hello, Rust.");
    }
}