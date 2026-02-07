use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::ratings::{AddMissionRatingEntity, MissionRatingSummary};

#[async_trait]
pub trait RatingRepository: Send + Sync {
    async fn add_rating(&self, rating: AddMissionRatingEntity) -> Result<i32>;
    async fn get_rating_by_mission_and_brawler(
        &self,
        mission_id: i32,
        brawler_id: i32,
    ) -> Result<Option<i32>>;
    async fn get_ratings_by_mission_id(&self, mission_id: i32) -> Result<MissionRatingSummary>;
}
