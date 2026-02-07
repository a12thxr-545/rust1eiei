use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::schema::mission_ratings;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = mission_ratings)]
pub struct MissionRatingEntity {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = mission_ratings)]
pub struct AddMissionRatingEntity {
    pub mission_id: i32,
    pub brawler_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionRatingModel {
    pub id: i32,
    pub mission_id: i32,
    pub brawler_id: i32,
    pub brawler_name: String,
    pub brawler_avatar: Option<String>,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionRatingSummary {
    pub average_rating: f64,
    pub total_ratings: i64,
    pub ratings: Vec<MissionRatingModel>,
}
