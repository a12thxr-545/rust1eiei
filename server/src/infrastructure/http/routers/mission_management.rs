use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};

use crate::{
    application::use_cases::mission_management::MissionManagementUseCase,
    domain::{
        repositories::{
            crew_operation::CrewOperationRepository,
            mission_management::MissionManagementRepository,
            mission_viewing::MissionViewingRepository,
        },
        value_objects::{
            mission_model::{AddMissionModel, EditMissionModel},
            uploaded_image::UploadedAvartar,
        },
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                crew_operation::CrewOperationPostgres,
                mission_management::MissionManagementPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
        realtime::RealtimeHub,
    },
};

pub async fn add<T1, T2, T3>(
    State(mission_management_use_case): State<Arc<MissionManagementUseCase<T1, T2, T3>>>,
    Extension(brawler_id): Extension<i32>,
    Json(add_mission_model): Json<AddMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    match mission_management_use_case
        .add(brawler_id, add_mission_model)
        .await
    {
        Ok(mission_id) => {
            let json_value = serde_json::json!({
             "mission_id": mission_id,
            });
            (StatusCode::CREATED, axum::Json(json_value)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn edit<T1, T2, T3>(
    State(mission_management_use_case): State<Arc<MissionManagementUseCase<T1, T2, T3>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(edit_mission_model): Json<EditMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    match mission_management_use_case
        .edit(mission_id, brawler_id, edit_mission_model)
        .await
    {
        Ok(mission_id) => {
            let response = format!("Edit mission success with id: {}", mission_id);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn remove<T1, T2, T3>(
    State(mission_management_use_case): State<Arc<MissionManagementUseCase<T1, T2, T3>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    match mission_management_use_case
        .remove(mission_id, brawler_id)
        .await
    {
        Ok(_) => {
            let response = format!("Remove mission success with id: {}", mission_id);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn upload_image<T1, T2, T3>(
    State(mission_management_use_case): State<Arc<MissionManagementUseCase<T1, T2, T3>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadedAvartar>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    match mission_management_use_case
        .upload_image(upload_image.base64_string, brawler_id)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(uploaded_image)).into_response(),
        Err(e) => {
            tracing::error!("Upload mission image error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>, realtime_hub: Arc<RealtimeHub>) -> Router {
    let mission_management_repository = MissionManagementPostgres::new(Arc::clone(&db_pool));
    let mission_viewing_repository = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));

    let mission_management_use_case = MissionManagementUseCase::new(
        Arc::new(mission_management_repository),
        Arc::new(mission_viewing_repository),
        Arc::new(crew_operation_repository),
        realtime_hub,
    );

    Router::new()
        .route("/", post(add))
        .route("/image", post(upload_image))
        .route("/{mission_id}", patch(edit))
        .route("/{mission_id}", delete(remove))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(mission_management_use_case))
}
