use diesel::{
    prelude::QueryableByName,
    sql_types::{BigInt, Int4, Varchar},
};
use serde::{Deserialize, Serialize};

use crate::domain::entities::brawlers::RegisterBrawlerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type = Int4)]
    pub brawler_id: i32,
    #[diesel(sql_type = Varchar)]
    pub display_name: String,
    #[diesel(sql_type = Varchar)]
    pub username: String,
    #[diesel(sql_type = Varchar)]
    pub avatar_url: String,
    #[diesel(sql_type = BigInt)]
    pub mission_success_count: i64,
    #[diesel(sql_type = BigInt)]
    pub mission_joined_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrawlerSummaryModel {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrawlerProfileModel {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationModel {
    pub current_page: i64,
    pub page_size: i64,
    pub length: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrawlerPaginationModel {
    pub pagination: PaginationModel,
    pub items: Vec<BrawlerSummaryModel>,
}
