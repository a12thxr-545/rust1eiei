use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{brawler_model::RegisterBrawlerModel, uploaded_image::UploadedAvartar},
    },
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        http::middleware::auth::authorization,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = BrawlerPostgres::new(db_pool);
    let brawlers_use_case = BrawlersUseCase::new(Arc::new(brawlers_repository));

    let protected_router = Router::new()
        .route("/avatar", post(upload_avatar))
        .route_layer(axum::middleware::from_fn(authorization));

    Router::new()
        .merge(protected_router)
        .route("/register", post(register))
        .with_state(Arc::new(brawlers_use_case))
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(passport) => (StatusCode::CREATED, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn upload_avatar<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadedAvartar>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .upload_avatar(upload_image.base64_string, brawler_id)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(uploaded_image)).into_response(),
        Err(e) => {
            tracing::error!("Upload avatar error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
