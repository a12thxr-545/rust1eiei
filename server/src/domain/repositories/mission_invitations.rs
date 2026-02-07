use crate::domain::entities::mission_invitations::{
    AddMissionInvitationEntity, MissionInvitationEntity,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MissionInvitationRepository {
    async fn invite(&self, entity: AddMissionInvitationEntity) -> Result<i32>;
    async fn accept(&self, invitation_id: i32) -> Result<()>;
    async fn reject(&self, invitation_id: i32) -> Result<()>;
    async fn get_received_invitations(&self, user_id: i32) -> Result<Vec<MissionInvitationEntity>>;
    async fn get_mission_invitations(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionInvitationEntity>>;
}
