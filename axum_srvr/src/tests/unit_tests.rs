#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::services;
    use std::sync::Arc;
use tokio::sync::Mutex;

    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_user() {
        let mut state = Arc::new(Mutex::new(services::user_service::AppState {
            db: services::user_service::DatabaseSim::new(),
        }));

        let app = routes::get_routes::get_user_route(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        print!("Status {}", &response.status());
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
