use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    dsl::count_star, insert_into,
};

use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            brawlers::BrawlerEntity,
            ratings::{
                AddMissionRatingEntity, MissionRatingEntity, MissionRatingModel,
                MissionRatingSummary,
            },
        },
        repositories::rating::RatingRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{brawlers, mission_ratings},
    },
};

pub struct RatingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl RatingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl RatingRepository for RatingPostgres {
    async fn add_rating(&self, rating: AddMissionRatingEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(mission_ratings::table)
            .values(rating)
            .returning(mission_ratings::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn get_rating_by_mission_and_brawler(
        &self,
        mission_id: i32,
        brawler_id: i32,
    ) -> Result<Option<i32>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = mission_ratings::table
            .filter(mission_ratings::mission_id.eq(mission_id))
            .filter(mission_ratings::brawler_id.eq(brawler_id))
            .select(mission_ratings::rating)
            .first::<i32>(&mut conn)
            .optional()?;
        Ok(result)
    }

    async fn get_ratings_by_mission_id(&self, mission_id: i32) -> Result<MissionRatingSummary> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        // Get count
        let total_ratings: i64 = mission_ratings::table
            .filter(mission_ratings::mission_id.eq(mission_id))
            .select(count_star())
            .first(&mut conn)?;

        // Get all ratings with brawler info
        let results = mission_ratings::table
            .inner_join(brawlers::table.on(mission_ratings::brawler_id.eq(brawlers::id)))
            .filter(mission_ratings::mission_id.eq(mission_id))
            .select((MissionRatingEntity::as_select(), BrawlerEntity::as_select()))
            .order(mission_ratings::created_at.desc())
            .load::<(MissionRatingEntity, BrawlerEntity)>(&mut conn)?;

        let ratings: Vec<MissionRatingModel> = results
            .into_iter()
            .map(|(rating, brawler)| MissionRatingModel {
                id: rating.id,
                mission_id: rating.mission_id,
                brawler_id: rating.brawler_id,
                brawler_name: brawler.display_name,
                brawler_avatar: brawler.avatar_url,
                rating: rating.rating,
                comment: rating.comment,
                created_at: rating.created_at,
            })
            .collect();

        // Calculate average manually
        let average_rating = if ratings.is_empty() {
            0.0
        } else {
            let sum: i32 = ratings.iter().map(|r| r.rating).sum();
            sum as f64 / ratings.len() as f64
        };

        Ok(MissionRatingSummary {
            average_rating,
            total_ratings,
            ratings,
        })
    }
}
