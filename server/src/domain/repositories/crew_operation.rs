use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::crew_memberships::CrewMemberShips;

#[async_trait]
pub trait CrewOperationRepository {
    async fn join(&self, crew_member_ships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_member_ships: CrewMemberShips) -> Result<()>;
    async fn get_current_mission(&self, brawler_id: i32) -> Result<Option<i32>>;
    async fn is_member(&self, mission_id: i32, brawler_id: i32) -> Result<bool>;
}
