use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::{
    application::use_cases::mission_viewing::MissionViewingUseCase,
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::mission_filter::MissionFilter,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::mission_viewing::MissionViewingPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_viewing_repository = MissionViewingPostgres::new(db_pool);
    let use_case = MissionViewingUseCase::new(Arc::new(mission_viewing_repository));

    Router::new()
        .route("/{mission_id}", get(view_details))
        .route("/gets", get(gets))
        .route("/count/{mission_id}", get(get_mission_count))
        .with_state(Arc::new(use_case))
}

pub async fn view_details<T>(
    State(mission_viewing_use_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match mission_viewing_use_case.get_one(mission_id).await {
        Ok(mission_model) => (StatusCode::OK, Json(mission_model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn gets<T>(
    State(mission_viewing_use_case): State<Arc<MissionViewingUseCase<T>>>,
    filter: Query<MissionFilter>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match mission_viewing_use_case.get_all(&filter).await {
        Ok(mission_models) => (StatusCode::OK, Json(mission_models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_mission_count<T>(
    State(mission_viewing_use_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match mission_viewing_use_case.get_mission_count(mission_id).await {
        Ok(brawler_models) => (StatusCode::OK, Json(brawler_models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}