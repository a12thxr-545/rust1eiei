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
    application::use_cases::chat::ChatUseCase,
    domain::repositories::{chat::ChatRepository, crew_operation::CrewOperationRepository},
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{chat::ChatPostgres, crew_operation::CrewOperationPostgres},
        },
        http::middleware::auth::authorization,
    },
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageModel {
    pub content: String,
    pub image_url: Option<String>,
}

pub async fn send_message<T1, T2>(
    State(chat_use_case): State<Arc<ChatUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(payload): Json<SendMessageModel>,
) -> impl IntoResponse
where
    T1: ChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    match chat_use_case
        .send_message(mission_id, brawler_id, payload.content, payload.image_url)
        .await
    {
        Ok(message_id) => (StatusCode::CREATED, Json(message_id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_messages<T1, T2>(
    State(chat_use_case): State<Arc<ChatUseCase<T1, T2>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    match chat_use_case.get_messages(mission_id, brawler_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let chat_repository = ChatPostgres::new(Arc::clone(&db_pool));
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));

    let chat_use_case = ChatUseCase::new(
        Arc::new(chat_repository),
        Arc::new(crew_operation_repository),
    );

    Router::new()
        .route("/{mission_id}", get(get_messages))
        .route("/{mission_id}", post(send_message))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(chat_use_case))
}
