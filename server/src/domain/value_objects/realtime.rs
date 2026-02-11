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
    FriendAccepted {
        from_id: i32,
        to_id: i32,
    },
    MissionInvitationAccepted {
        mission_id: i32,
        user_id: i32,
        inviter_id: i32,
    },
    MissionStatusChanged {
        mission_id: i32,
        status: String,
        brawler_id: i32,
    },
    MissionDeleted {
        mission_id: i32,
    },
    MissionCreated {
        mission_id: i32,
        chief_id: i32,
    },
    MissionUpdated {
        mission_id: i32,
        chief_id: i32,
    },
    MissionJoined {
        mission_id: i32,
        brawler_id: i32,
    },
    MissionLeft {
        mission_id: i32,
        brawler_id: i32,
    },
}
