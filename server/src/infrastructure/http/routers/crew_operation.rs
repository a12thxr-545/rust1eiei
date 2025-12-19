use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
};

use crate::{
    application::use_cases::crew_operation::CrewOperationUseCase,
    config::config_model::DotEnvyConfig,
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
        http::middlewares::auth::auth,
    },
};

pub async fn join<T1, T2>(
    State(use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.join(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Join mission_id: {} completed!!", mission_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn leave<T1, T2>(
    State(use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.leave(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Leave mission_id: {} completed!!", mission_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>, config: Arc<DotEnvyConfig>) -> Router {
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let mission_viewing_repository = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let use_case = CrewOperationUseCase::new(
        Arc::new(crew_operation_repository),
        Arc::new(mission_viewing_repository),
        config.server.max_crew_per_mission,
    );

    Router::new()
        .route("/join/{mission_id}", post(join))
        .route("/leave/{mission_id}", delete(leave))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(use_case))
}
