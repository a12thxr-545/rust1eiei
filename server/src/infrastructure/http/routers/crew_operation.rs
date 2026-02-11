use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};

use crate::{
    application::use_cases::crew_operation::CrewOperationUseCase,
    domain::repositories::{
        crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                crew_operation::CrewOperationPostgres, mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
        realtime::RealtimeHub,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>, realtime_hub: Arc<RealtimeHub>) -> Router {
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let mission_viewing_repository = MissionViewingPostgres::new(Arc::clone(&db_pool));

    let use_case = CrewOperationUseCase::new(
        Arc::new(crew_operation_repository),
        Arc::new(mission_viewing_repository),
        realtime_hub,
    );

    Router::new()
        .route("/join/{mission_id}", post(join))
        .route("/leave/{mission_id}", delete(leave))
        .route("/current", get(current_mission))
        .route("/kick/{mission_id}/{brawler_id}", delete(kick))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}

pub async fn join<T1, T2>(
    State(crew_operation_use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match crew_operation_use_case.join(mission_id, brawler_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Brawler id: {}, has joined mission id: {}",
                brawler_id, mission_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn leave<T1, T2>(
    State(crew_operation_use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match crew_operation_use_case.leave(mission_id, brawler_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Brawler id: {}, has left mission id: {}",
                brawler_id, mission_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn current_mission<T1, T2>(
    State(crew_operation_use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match crew_operation_use_case
        .get_current_mission(brawler_id)
        .await
    {
        Ok(mission_id) => {
            let json = serde_json::json!({
                "mission_id": mission_id
            });
            (StatusCode::OK, Json(json)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
pub async fn kick<T1, T2>(
    State(crew_operation_use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path((mission_id, brawler_id)): Path<(i32, i32)>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match crew_operation_use_case
        .kick(mission_id, chief_id, brawler_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Brawler id: {}, has been kicked from mission id: {}",
                brawler_id, mission_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
