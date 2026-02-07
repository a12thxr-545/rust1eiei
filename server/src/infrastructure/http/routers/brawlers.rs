use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use serde::Deserialize;

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub query: Option<String>,
    pub current_page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = BrawlerPostgres::new(db_pool);
    let brawlers_use_case = BrawlersUseCase::new(Arc::new(brawlers_repository));

    let protected_router = Router::new()
        .route("/avatar", post(upload_avatar))
        .route("/cover", post(upload_cover))
        .route("/chat-image", post(upload_chat_image))
        .route("/profile", get(get_profile))
        .route("/display-name", put(update_display_name))
        .route("/search", get(search))
        .route("/{username}", get(get_profile_by_username))
        .route_layer(axum::middleware::from_fn(authorization));

    Router::new()
        .merge(protected_router)
        .route("/register", post(register))
        .route("/check-username/{username}", get(check_username))
        .with_state(Arc::new(brawlers_use_case))
}

pub async fn check_username<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    axum::extract::Path(username): axum::extract::Path<String>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.check_username(username).await {
        Ok(available) => (StatusCode::OK, Json(available)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn search<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    let query = params.query.unwrap_or_default();
    let page = params.current_page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match brawlers_use_case.search(&query, page, page_size).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => {
            tracing::error!("Search brawlers error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
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
        Err(e) => {
            let status = if e.to_string().contains("already taken") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, e.to_string()).into_response()
        }
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

pub async fn upload_cover<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadedAvartar>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .upload_cover(upload_image.base64_string, brawler_id)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(uploaded_image)).into_response(),
        Err(e) => {
            tracing::error!("Upload cover error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn upload_chat_image<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadedAvartar>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .upload_chat_image(upload_image.base64_string, brawler_id)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(uploaded_image)).into_response(),
        Err(e) => {
            tracing::error!("Upload chat image error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn get_profile<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_profile(brawler_id).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => {
            tracing::error!("Get profile error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn get_profile_by_username<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    axum::extract::Path(username): axum::extract::Path<String>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_profile_by_username(username).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => {
            tracing::error!("Get profile by username error: {:?}", e);
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDisplayNameRequest {
    pub display_name: String,
}

pub async fn update_display_name<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(request): Json<UpdateDisplayNameRequest>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .update_display_name(brawler_id, request.display_name)
        .await
    {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => {
            tracing::error!("Update display name error: {:?}", e);
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
