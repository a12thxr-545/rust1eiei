use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};

use crate::{
    application::use_cases::mission_operation::MissionOperationUseCase,
    domain::{
        repositories::{
            mission_operation::MissionOperationRepository,
            mission_viewing::MissionViewingRepository,
        },
        value_objects::mission_statuses::MissionStatuses,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                mission_operation::MissionOperationPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_operation_repository = MissionOperationPostgres::new(Arc::clone(&db_pool));
    let mission_viewing_repository = MissionViewingPostgres::new(Arc::clone(&db_pool));

    let use_case = MissionOperationUseCase::new(
        Arc::new(mission_operation_repository),
        Arc::new(mission_viewing_repository),
    );

    Router::new()
        .route("/in-progress/{mission_id}", patch(in_progress))
        .route("/to-completed/{mission_id}", patch(to_completed))
        .route("/to-failed/{mission_id}", patch(to_failed))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}

pub async fn in_progress<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case
        .in_progress(mission_id, chief_id)
        .await
    {
        Ok(mission_id) => {
            let response = format!(
                "Mision ({}) is now {:?}",
                mission_id,
                MissionStatuses::InProgress
            );
            (StatusCode::CREATED, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_completed<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case
        .to_completed(mission_id, chief_id)
        .await
    {
        Ok(quest_id) => (
            StatusCode::OK,
            format!(
                "Mission ({}) is now {:?}",
                quest_id,
                MissionStatuses::Completed
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case
        .to_failed(mission_id, chief_id)
        .await
    {
        Ok(mission_id) => (
            StatusCode::OK,
            format!(
                "Mission ({}) is now {:?}",
                mission_id,
                MissionStatuses::Failed
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
