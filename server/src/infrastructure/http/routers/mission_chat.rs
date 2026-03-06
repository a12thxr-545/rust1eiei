use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    application::use_cases::mission_chat::MissionChatUseCase,
    domain::repositories::{
        brawlers::BrawlerRepository, crew_operation::CrewOperationRepository,
        mission_chat::MissionChatRepository, mission_viewing::MissionViewingRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                brawlers::BrawlerPostgres, crew_operation::CrewOperationPostgres,
                mission_chat::MissionChatPostgres, mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
        realtime::RealtimeHub,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>, realtime_hub: Arc<RealtimeHub>) -> Router {
    let mission_chat_repo = MissionChatPostgres::new(Arc::clone(&db_pool));
    let crew_repo = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let mission_view_repo = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let brawler_repo = BrawlerPostgres::new(Arc::clone(&db_pool));

    let use_case = MissionChatUseCase::new(
        Arc::new(mission_chat_repo),
        Arc::new(crew_repo),
        Arc::new(mission_view_repo),
        Arc::new(brawler_repo),
        realtime_hub,
    );

    Router::new()
        .route("/{mission_id}", get(get_messages))
        .route("/{mission_id}", post(send_message))
        .layer(axum::middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub content: String,
}

pub async fn send_message<T1, T2, T3, T4>(
    State(use_case): State<Arc<MissionChatUseCase<T1, T2, T3, T4>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse
where
    T1: MissionChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
    T3: MissionViewingRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match use_case
        .send_message(mission_id, brawler_id, payload.content)
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_messages<T1, T2, T3, T4>(
    State(use_case): State<Arc<MissionChatUseCase<T1, T2, T3, T4>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
    T3: MissionViewingRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match use_case.get_messages(mission_id, brawler_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
