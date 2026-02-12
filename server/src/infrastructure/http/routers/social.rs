use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{delete, get, post},
};
use futures::stream::Stream;
use serde::Deserialize;
use std::{convert::Infallible, sync::Arc};
use tokio_stream::StreamExt;

use crate::{
    application::use_cases::social::SocialUseCase,
    domain::{
        repositories::{
            brawlers::BrawlerRepository, crew_operation::CrewOperationRepository,
            friendships::FriendshipRepository, mission_invitations::MissionInvitationRepository,
            mission_viewing::MissionViewingRepository,
        },
        value_objects::social_model::FriendshipStatusModel,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                brawlers::BrawlerPostgres, crew_operation::CrewOperationPostgres,
                friendships::FriendshipPostgres, mission_invitations::MissionInvitationPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::{authorization, optional_authorization},
        realtime::RealtimeHub,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>, realtime_hub: Arc<RealtimeHub>) -> Router {
    let friendship_repo = FriendshipPostgres::new(Arc::clone(&db_pool));
    let invitation_repo = MissionInvitationPostgres::new(Arc::clone(&db_pool));
    let brawlers_repo = BrawlerPostgres::new(Arc::clone(&db_pool));
    let crew_repo = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let mission_repo = MissionViewingPostgres::new(Arc::clone(&db_pool));

    let use_case = SocialUseCase::new(
        Arc::new(friendship_repo),
        Arc::new(invitation_repo),
        Arc::new(brawlers_repo),
        Arc::new(mission_repo),
        Arc::new(crew_repo),
        Arc::clone(&realtime_hub),
    );

    let protected_routes = Router::new()
        .route("/events", get(get_realtime_events))
        .route("/friends", get(get_friends))
        .route("/friends/requests", get(get_pending_requests))
        .route("/friends/add/{friend_id}", post(add_friend))
        .route("/friends/accept/{friend_id}", post(accept_friend))
        .route("/friends/reject/{friend_id}", delete(reject_friend))
        .route("/friends/remove/{friend_id}", delete(remove_friend))
        .route("/invite/{invitee_id}/{mission_id}", post(invite_to_mission))
        .route("/invitations", get(get_my_invitations))
        .route(
            "/mission/{mission_id}/invitations",
            get(get_mission_invitations),
        )
        .route(
            "/invitations/respond/{invitation_id}",
            post(respond_to_invitation),
        )
        .route_layer(axum::middleware::from_fn(authorization));

    Router::new()
        .route(
            "/status/{other_id}",
            get(get_friendship_status).layer(axum::middleware::from_fn(optional_authorization)),
        )
        .merge(protected_routes)
        .with_state(Arc::new(use_case))
}

#[derive(Deserialize)]
pub struct RespondInvitation {
    pub accept: bool,
}

pub async fn get_friends<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.get_friends(user_id).await {
        Ok(friends) => (StatusCode::OK, Json(friends)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_pending_requests<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.get_pending_requests(user_id).await {
        Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn add_friend<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.add_friend(user_id, friend_id).await {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn accept_friend<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.accept_friend(user_id, friend_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn reject_friend<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.reject_friend(user_id, friend_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn invite_to_mission<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
    Path((invitee_id, mission_id)): Path<(i32, i32)>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case
        .invite_to_mission(user_id, invitee_id, mission_id)
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_my_invitations<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.get_my_invitations(user_id).await {
        Ok(invitations) => (StatusCode::OK, Json(invitations)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn respond_to_invitation<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
    Path(invitation_id): Path<i32>,
    Json(payload): Json<RespondInvitation>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case
        .respond_to_invitation(user_id, invitation_id, payload.accept)
        .await
    {
        Ok(mid) => (
            StatusCode::OK,
            Json(serde_json::json!({ "mission_id": mid })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_mission_invitations<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(_user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    match use_case.get_mission_invitations(mission_id).await {
        Ok(invitations) => (StatusCode::OK, Json(invitations)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_friendship_status<T1, T2, T3, T4, T5>(
    Path(other_id): Path<i32>,
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    user_id_ext: Option<Extension<i32>>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    let user_id = user_id_ext.map(|Extension(id)| id).unwrap_or(0);

    if user_id == 0 {
        return (
            StatusCode::OK,
            Json(FriendshipStatusModel {
                friendship_id: None,
                initiator_id: None,
                status: "none".to_string(),
            }),
        )
            .into_response();
    }

    match use_case.get_friendship_status(user_id, other_id).await {
        Ok(status) => (StatusCode::OK, Json(status)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove_friend<T1, T2, T3, T4, T5>(
    Path(friend_id): Path<i32>,
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    tracing::info!(
        "Removing friendship: user_id={}, friend_id={}",
        user_id,
        friend_id
    );
    match use_case.remove_friend(user_id, friend_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to remove friendship: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn get_realtime_events<T1, T2, T3, T4, T5>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4, T5>>>,
    user_id_ext: Option<Extension<i32>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>>
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    let user_id = user_id_ext.map(|Extension(id)| id).unwrap_or(0);

    if user_id == 0 {
        tracing::error!("Realtime connection attempted without authorization");
    }

    tracing::info!("User {} connected to realtime events", user_id);
    let rx = use_case.realtime_hub.tx.subscribe();
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(move |event| {
        if let Ok(event) = event {
            // Filter events relevant to this user
            let is_relevant = match &event {
                crate::domain::value_objects::realtime::RealtimeEvent::FriendRequest {
                    to_id,
                    ..
                } => *to_id == user_id,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionInvitation {
                    invitee_id,
                    ..
                } => *invitee_id == user_id,
                crate::domain::value_objects::realtime::RealtimeEvent::FriendAccepted {
                    to_id,
                    ..
                } => *to_id == user_id,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionInvitationAccepted {
                    inviter_id,
                    ..
                } => *inviter_id == user_id,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionStatusChanged {
                    ..
                } => true,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionDeleted {
                    ..
                } => true,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionCreated {
                    ..
                } => true,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionUpdated {
                    ..
                } => true,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionJoined {
                    ..
                } => true,
                crate::domain::value_objects::realtime::RealtimeEvent::MissionLeft {
                    ..
                } => true,
            };

            if is_relevant {
                let data = serde_json::to_string(&event).unwrap();
                return Some(Ok(Event::default().data(data)));
            }
        }
        None
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
