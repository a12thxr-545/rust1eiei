use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    application::use_cases::rating::RatingUseCase,
    domain::repositories::{crew_operation::CrewOperationRepository, rating::RatingRepository},
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{crew_operation::CrewOperationPostgres, rating::RatingPostgres},
        },
        http::middleware::auth::authorization,
    },
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRatingRequest {
    pub rating: i32,
    pub comment: Option<String>,
}

pub async fn add_rating<T1, T2>(
    State(rating_use_case): State<Arc<RatingUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(payload): Json<AddRatingRequest>,
) -> impl IntoResponse
where
    T1: RatingRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    match rating_use_case
        .add_rating(mission_id, brawler_id, payload.rating, payload.comment)
        .await
    {
        Ok(rating_id) => (StatusCode::CREATED, Json(rating_id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_mission_ratings<T1, T2>(
    State(rating_use_case): State<Arc<RatingUseCase<T1, T2>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: RatingRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    match rating_use_case.get_mission_ratings(mission_id).await {
        Ok(summary) => (StatusCode::OK, Json(summary)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_user_rating<T1, T2>(
    State(rating_use_case): State<Arc<RatingUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: RatingRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    match rating_use_case
        .get_user_rating(mission_id, brawler_id)
        .await
    {
        Ok(rating) => (StatusCode::OK, Json(rating)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let rating_repository = RatingPostgres::new(Arc::clone(&db_pool));
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));

    let rating_use_case = RatingUseCase::new(
        Arc::new(rating_repository),
        Arc::new(crew_operation_repository),
    );

    Router::new()
        .route("/{mission_id}", get(get_mission_ratings))
        .route("/{mission_id}", post(add_rating))
        .route("/{mission_id}/my-rating", get(get_user_rating))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(rating_use_case))
}
