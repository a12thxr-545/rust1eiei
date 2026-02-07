use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::chat_messages::{
    AddMissionChatMessageEntity, MissionChatMessageModel,
};

#[async_trait]
pub trait ChatRepository {
    async fn add(&self, add_chat_message_entity: AddMissionChatMessageEntity) -> Result<i32>;
    async fn get_messages_by_mission_id(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionChatMessageModel>>;
}
