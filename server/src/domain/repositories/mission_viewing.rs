use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{
    entities::missions::MissionEntity,
    value_objects::{brawler_model::BrawlerModel, mission_filter::MissionFilter},
};

#[async_trait]
pub trait MissionViewingRepository {
    async fn crew_counting(&self, mission_id: i32) -> Result<i64>;
    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity>;
    async fn get_all(&self, mission_filter: &MissionFilter) -> Result<Vec<MissionEntity>>;
    async fn get_mission_count(&self, mission_id: i32) -> Result<Vec<BrawlerModel>>;
    async fn get_chief_name(&self, chief_id: i32) -> Result<String>;
}
