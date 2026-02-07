use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    application::use_cases::social::SocialUseCase,
    domain::repositories::{
        brawlers::BrawlerRepository, friendships::FriendshipRepository,
        mission_invitations::MissionInvitationRepository,
        mission_viewing::MissionViewingRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                brawlers::BrawlerPostgres, friendships::FriendshipPostgres,
                mission_invitations::MissionInvitationPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let friendship_repo = FriendshipPostgres::new(Arc::clone(&db_pool));
    let invitation_repo = MissionInvitationPostgres::new(Arc::clone(&db_pool));
    let brawlers_repo = BrawlerPostgres::new(Arc::clone(&db_pool));
    let mission_repo = MissionViewingPostgres::new(Arc::clone(&db_pool));

    let use_case = SocialUseCase::new(
        Arc::new(friendship_repo),
        Arc::new(invitation_repo),
        Arc::new(brawlers_repo),
        Arc::new(mission_repo),
    );

    Router::new()
        .route("/friends", get(get_friends))
        .route("/friends/requests", get(get_pending_requests))
        .route("/friends/add/{friend_id}", post(add_friend))
        .route("/friends/accept/{friend_id}", post(accept_friend))
        .route("/friends/reject/{friend_id}", delete(reject_friend))
        .route("/invite/{invitee_id}/{mission_id}", post(invite_to_mission))
        .route("/invitations", get(get_my_invitations))
        .route(
            "/invitations/respond/{invitation_id}",
            post(respond_to_invitation),
        )
        .route_layer(axum::middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}

#[derive(Deserialize)]
pub struct RespondInvitation {
    pub accept: bool,
}

pub async fn get_friends<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.get_friends(user_id).await {
        Ok(friends) => (StatusCode::OK, Json(friends)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_pending_requests<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.get_pending_requests(user_id).await {
        Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn add_friend<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.add_friend(user_id, friend_id).await {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn accept_friend<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.accept_friend(user_id, friend_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn reject_friend<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.reject_friend(user_id, friend_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn invite_to_mission<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path((invitee_id, mission_id)): Path<(i32, i32)>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case
        .invite_to_mission(user_id, invitee_id, mission_id)
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_my_invitations<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case.get_my_invitations(user_id).await {
        Ok(invitations) => (StatusCode::OK, Json(invitations)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn respond_to_invitation<T1, T2, T3, T4>(
    State(use_case): State<Arc<SocialUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(invitation_id): Path<i32>,
    Json(payload): Json<RespondInvitation>,
) -> impl IntoResponse
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    match use_case
        .respond_to_invitation(user_id, invitation_id, payload.accept)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
