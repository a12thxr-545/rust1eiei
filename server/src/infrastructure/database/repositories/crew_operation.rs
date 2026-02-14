use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    dsl::delete, insert_into,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::crew_memberships::CrewMemberShips,
        repositories::crew_operation::CrewOperationRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{crew_memberships, missions},
    },
};

pub struct CrewOperationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewOperationPostgres {
    async fn join(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        insert_into(crew_memberships::table)
            .values(crew_member_ships)
            .execute(&mut conn)?;
        Ok(())
    }

    async fn leave(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_member_ships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_member_ships.mission_id))
            .execute(&mut conn)?;
        Ok(())
    }

    async fn get_current_mission(&self, brawler_id: i32) -> Result<Option<i32>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        // 1. Check if user is a crew member in any active mission
        let crew_mission_id = crew_memberships::table
            .inner_join(missions::table)
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .filter(missions::deleted_at.is_null())
            .filter(
                missions::status
                    .eq("Open")
                    .or(missions::status.eq("InProgress")),
            )
            .select(crew_memberships::mission_id)
            .first::<i32>(&mut conn)
            .optional()?;

        if crew_mission_id.is_some() {
            return Ok(crew_mission_id);
        }

        // 2. Check if user is the chief of any active mission
        let chief_mission_id = missions::table
            .filter(missions::chief_id.eq(brawler_id))
            .filter(missions::deleted_at.is_null())
            .filter(
                missions::status
                    .eq("Open")
                    .or(missions::status.eq("InProgress")),
            )
            .select(missions::id)
            .first::<i32>(&mut conn)
            .optional()?;

        Ok(chief_mission_id)
    }

    async fn is_member(&self, mission_id: i32, brawler_id: i32) -> Result<bool> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let exists = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .first::<(i32, i32, chrono::NaiveDateTime)>(&mut conn)
            .optional()?
            .is_some();
        Ok(exists)
    }
}
