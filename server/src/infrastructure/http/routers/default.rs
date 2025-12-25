use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, " All Right, I'am Good").into_response()
}
