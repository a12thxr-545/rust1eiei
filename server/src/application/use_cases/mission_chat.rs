use anyhow::{Result, bail};
use chrono::Utc;
use std::sync::Arc;

use crate::{
    domain::{
        entities::mission_chat::{MissionChatMessageWithBrawler, NewMissionChatMessageEntity},
        repositories::{
            brawlers::BrawlerRepository, crew_operation::CrewOperationRepository,
            mission_chat::MissionChatRepository, mission_viewing::MissionViewingRepository,
        },
        value_objects::realtime::RealtimeEvent,
    },
    infrastructure::realtime::RealtimeHub,
};

pub struct MissionChatUseCase<T1, T2, T3, T4> {
    mission_chat_repo: Arc<T1>,
    crew_repo: Arc<T2>,
    mission_view_repo: Arc<T3>,
    brawler_repo: Arc<T4>,
    realtime_hub: Arc<RealtimeHub>,
}

impl<T1, T2, T3, T4> MissionChatUseCase<T1, T2, T3, T4>
where
    T1: MissionChatRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
    T3: MissionViewingRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    pub fn new(
        mission_chat_repo: Arc<T1>,
        crew_repo: Arc<T2>,
        mission_view_repo: Arc<T3>,
        brawler_repo: Arc<T4>,
        realtime_hub: Arc<RealtimeHub>,
    ) -> Self {
        Self {
            mission_chat_repo,
            crew_repo,
            mission_view_repo,
            brawler_repo,
            realtime_hub,
        }
    }

    pub async fn send_message(
        &self,
        mission_id: i32,
        brawler_id: i32,
        content: String,
    ) -> Result<i32> {
        // Authorization: Check if user is in mission
        let is_member = self.crew_repo.is_member(mission_id, brawler_id).await?;
        let mission = self.mission_view_repo.get_one(mission_id).await?;
        let is_chief = mission.chief_id == brawler_id;

        if !is_member && !is_chief {
            bail!("You are not a member of this mission");
        }

        let brawler = self.brawler_repo.find_by_id(brawler_id).await?;

        let now = Utc::now().naive_utc();
        let new_message = NewMissionChatMessageEntity {
            mission_id,
            brawler_id,
            content: content.clone(),
            created_at: now,
            image_url: None,
        };

        let message_id = self.mission_chat_repo.save_message(new_message).await?;

        // Broadcast
        self.realtime_hub
            .broadcast(RealtimeEvent::MissionChatMessage {
                mission_id,
                brawler_id,
                brawler_name: brawler.display_name,
                content,
                created_at: now,
            });

        Ok(message_id)
    }

    pub async fn get_messages(
        &self,
        mission_id: i32,
        brawler_id: i32,
    ) -> Result<Vec<MissionChatMessageWithBrawler>> {
        // Authorization: Check if user is in mission
        let is_member = self.crew_repo.is_member(mission_id, brawler_id).await?;
        let mission = self.mission_view_repo.get_one(mission_id).await?;
        let is_chief = mission.chief_id == brawler_id;

        if !is_member && !is_chief {
            bail!("You are not a member of this mission");
        }

        self.mission_chat_repo
            .get_messages_by_mission(mission_id)
            .await
    }
}
