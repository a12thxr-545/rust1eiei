use crate::infrastructure::database::schema::mission_chat_messages;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = mission_chat_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MissionChatMessageEntity {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mission_chat_messages)]
pub struct NewMissionChatMessageEntity {
    pub mission_id: i32,
    pub brawler_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionChatMessageWithBrawler {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub brawler_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub image_url: Option<String>,
}
