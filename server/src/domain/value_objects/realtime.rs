use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RealtimeEvent {
    FriendRequest {
        from_id: i32,
        to_id: i32,
    },
    MissionInvitation {
        mission_id: i32,
        inviter_id: i32,
        invitee_id: i32,
    },
}
