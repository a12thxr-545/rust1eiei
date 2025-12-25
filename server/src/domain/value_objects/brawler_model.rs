use diesel::{prelude::QueryableByName, sql_types::{BigInt, Varchar}};
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

#[derive(Debug, Clone, Serialize,Deserialize, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type = Varchar)]
    pub display_name: String,
    #[diesel(sql_type = Varchar)]
    pub avatar_url: String,
    #[diesel(sql_type = BigInt)]
    pub mission_success_count: i64,
    #[diesel(sql_type = BigInt)]
    pub mission_joined_count: i64,
}