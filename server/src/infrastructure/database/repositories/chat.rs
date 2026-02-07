use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper, insert_into};
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            brawlers::BrawlerEntity,
            chat_messages::{
                AddMissionChatMessageEntity, MissionChatMessageEntity, MissionChatMessageModel,
            },
        },
        repositories::chat::ChatRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{brawlers, mission_chat_messages},
    },
};

pub struct ChatPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl ChatPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ChatRepository for ChatPostgres {
    async fn add(&self, add_chat_message_entity: AddMissionChatMessageEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(mission_chat_messages::table)
            .values(add_chat_message_entity)
            .returning(mission_chat_messages::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn get_messages_by_mission_id(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionChatMessageModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let results = mission_chat_messages::table
            .inner_join(brawlers::table.on(mission_chat_messages::brawler_id.eq(brawlers::id)))
            .filter(mission_chat_messages::mission_id.eq(mission_id))
            .select((
                MissionChatMessageEntity::as_select(),
                BrawlerEntity::as_select(),
            ))
            .load::<(MissionChatMessageEntity, BrawlerEntity)>(&mut conn)?;

        let models = results
            .into_iter()
            .map(|(msg, brawler)| MissionChatMessageModel {
                id: msg.id,
                mission_id: msg.mission_id,
                brawler_id: msg.brawler_id,
                brawler_name: brawler.display_name,
                brawler_avatar: brawler.avatar_url,
                content: msg.content,
                created_at: msg.created_at,
                image_url: msg.image_url,
            })
            .collect();

        Ok(models)
    }
}
