use crate::domain::entities::mission_chat::{
    MissionChatMessageWithBrawler, NewMissionChatMessageEntity,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MissionChatRepository {
    async fn save_message(&self, message: NewMissionChatMessageEntity) -> Result<i32>;
    async fn get_messages_by_mission(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionChatMessageWithBrawler>>;
}
