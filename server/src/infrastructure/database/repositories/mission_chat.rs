use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;

use crate::{
    domain::{
        entities::mission_chat::{MissionChatMessageWithBrawler, NewMissionChatMessageEntity},
        repositories::mission_chat::MissionChatRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{brawlers, mission_chat_messages},
    },
};

pub struct MissionChatPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionChatPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionChatRepository for MissionChatPostgres {
    async fn save_message(&self, message: NewMissionChatMessageEntity) -> Result<i32> {
        let mut conn = self.db_pool.get()?;
        let id = diesel::insert_into(mission_chat_messages::table)
            .values(&message)
            .returning(mission_chat_messages::id)
            .get_result::<i32>(&mut conn)?;
        Ok(id)
    }

    async fn get_messages_by_mission(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionChatMessageWithBrawler>> {
        let mut conn = self.db_pool.get()?;

        let results = mission_chat_messages::table
            .inner_join(brawlers::table)
            .filter(mission_chat_messages::mission_id.eq(mission_id))
            .order_by(mission_chat_messages::created_at.asc())
            .select((
                mission_chat_messages::id,
                mission_chat_messages::mission_id,
                mission_chat_messages::brawler_id,
                brawlers::display_name,
                mission_chat_messages::content,
                mission_chat_messages::created_at,
                mission_chat_messages::image_url,
            ))
            .load::<(
                i32,
                i32,
                i32,
                String,
                String,
                chrono::NaiveDateTime,
                Option<String>,
            )>(&mut conn)?;

        let messages = results
            .into_iter()
            .map(
                |(id, m_id, b_id, name, content, created, img)| MissionChatMessageWithBrawler {
                    id,
                    mission_id: m_id,
                    brawler_id: b_id,
                    brawler_name: name,
                    content,
                    created_at: chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                        created,
                        chrono::Utc,
                    ),
                    image_url: img,
                },
            )
            .collect();

        Ok(messages)
    }
}
