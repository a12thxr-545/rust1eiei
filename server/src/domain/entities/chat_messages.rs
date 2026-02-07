use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::schema::mission_chat_messages;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = mission_chat_messages)]
pub struct MissionChatMessageEntity {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = mission_chat_messages)]
pub struct AddMissionChatMessageEntity {
    pub mission_id: i32,
    pub brawler_id: i32,
    pub content: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionChatMessageModel {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub brawler_name: String,
    pub brawler_avatar: Option<String>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub image_url: Option<String>,
}
