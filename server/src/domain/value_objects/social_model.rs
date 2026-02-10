use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendModel {
    pub friendship_id: i32,
    pub friend_id: i32,
    pub display_name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionInvitationModel {
    pub invitation_id: i32,
    pub mission_id: i32,
    pub mission_name: String,
    pub inviter_id: i32,
    pub inviter_name: String,
    pub invitee_id: i32,
    pub invitee_name: String,
    pub status: String,
    pub created_at: String,
}
