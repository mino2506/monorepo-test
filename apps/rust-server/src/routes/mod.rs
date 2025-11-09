use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

mod echo;
mod health;
mod hello;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::handler))
        .route("/hello", get(hello::handler))
        .route("/echo", post(echo::handler))
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn health_route_works() {
        let app = router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn echo_route_exists() {
        let app = router();

        let req_body = r#"{"text":"hi"}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/echo")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(req_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
