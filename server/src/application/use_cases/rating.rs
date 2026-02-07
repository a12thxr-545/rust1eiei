use anyhow::{Result, anyhow};
use std::sync::Arc;

use crate::domain::{
    entities::ratings::{AddMissionRatingEntity, MissionRatingSummary},
    repositories::{crew_operation::CrewOperationRepository, rating::RatingRepository},
};

pub struct RatingUseCase<T1, T2>
where
    T1: RatingRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    rating_repository: Arc<T1>,
    crew_operation_repository: Arc<T2>,
}

impl<T1, T2> RatingUseCase<T1, T2>
where
    T1: RatingRepository + Send + Sync,
    T2: CrewOperationRepository + Send + Sync,
{
    pub fn new(rating_repository: Arc<T1>, crew_operation_repository: Arc<T2>) -> Self {
        Self {
            rating_repository,
            crew_operation_repository,
        }
    }

    pub async fn add_rating(
        &self,
        mission_id: i32,
        brawler_id: i32,
        rating: i32,
        comment: Option<String>,
    ) -> Result<i32> {
        // Validate rating range
        if rating < 1 || rating > 5 {
            return Err(anyhow!("Rating must be between 1 and 5"));
        }

        // Check if user was/is a member of this mission
        let current_mission = self
            .crew_operation_repository
            .get_current_mission(brawler_id)
            .await?;

        // For now, allow rating if the user is currently in the mission
        // In production, you might want to check if they were ever in the mission
        if current_mission != Some(mission_id) {
            return Err(anyhow!("You must be a member of this mission to rate it"));
        }

        // Check if user already rated this mission
        let existing_rating = self
            .rating_repository
            .get_rating_by_mission_and_brawler(mission_id, brawler_id)
            .await?;

        if existing_rating.is_some() {
            return Err(anyhow!("You have already rated this mission"));
        }

        let add_rating = AddMissionRatingEntity {
            mission_id,
            brawler_id,
            rating,
            comment,
        };

        self.rating_repository.add_rating(add_rating).await
    }

    pub async fn get_mission_ratings(&self, mission_id: i32) -> Result<MissionRatingSummary> {
        self.rating_repository
            .get_ratings_by_mission_id(mission_id)
            .await
    }

    pub async fn get_user_rating(&self, mission_id: i32, brawler_id: i32) -> Result<Option<i32>> {
        self.rating_repository
            .get_rating_by_mission_and_brawler(mission_id, brawler_id)
            .await
    }
}
