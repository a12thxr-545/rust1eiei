use anyhow::{Result, anyhow};
use std::sync::Arc;

use crate::domain::{
    entities::chat_messages::{AddMissionChatMessageEntity, MissionChatMessageModel},
    repositories::{chat::ChatRepository, crew_operation::CrewOperationRepository},
};

pub struct ChatUseCase<T1, T2>
where
    T1: ChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    chat_repository: Arc<T1>,
    crew_operation_repository: Arc<T2>,
}

impl<T1, T2> ChatUseCase<T1, T2>
where
    T1: ChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    pub fn new(chat_repository: Arc<T1>, crew_operation_repository: Arc<T2>) -> Self {
        Self {
            chat_repository,
            crew_operation_repository,
        }
    }

    pub async fn send_message(
        &self,
        mission_id: i32,
        brawler_id: i32,
        content: String,
        image_url: Option<String>,
    ) -> Result<i32> {
        // Only members of the mission can send messages
        let current_mission = self
            .crew_operation_repository
            .get_current_mission(brawler_id)
            .await?;

        match current_mission {
            Some(id) if id == mission_id => {
                let add_msg = AddMissionChatMessageEntity {
                    mission_id,
                    brawler_id,
                    content,
                    image_url,
                };
                self.chat_repository.add(add_msg).await
            }
            _ => Err(anyhow!(
                "You must be a member of this mission to send messages."
            )),
        }
    }

    pub async fn get_messages(
        &self,
        mission_id: i32,
        brawler_id: i32,
    ) -> Result<Vec<MissionChatMessageModel>> {
        // Only members of the mission can see messages
        let current_mission = self
            .crew_operation_repository
            .get_current_mission(brawler_id)
            .await?;

        match current_mission {
            Some(id) if id == mission_id => {
                self.chat_repository
                    .get_messages_by_mission_id(mission_id)
                    .await
            }
            _ => Err(anyhow!(
                "You must be a member of this mission to view messages."
            )),
        }
    }
}
