use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::schema::mission_invitations;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = mission_invitations)]
pub struct MissionInvitationEntity {
    pub id: i32,
    pub mission_id: i32,
    pub inviter_id: i32,
    pub invitee_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = mission_invitations)]
pub struct AddMissionInvitationEntity {
    pub mission_id: i32,
    pub inviter_id: i32,
    pub invitee_id: i32,
    pub status: String,
}
