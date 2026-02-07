use crate::{
    domain::{
        entities::missions::{AddMissionEntity, EditMissionEntity},
        repositories::mission_management::MissionManagementRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::missions},
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{
    ExpressionMethods, OptionalExtension, RunQueryDsl, dsl::now, dsl::update, insert_into,
};
use std::sync::Arc;

pub struct MissionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MissionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(missions::table)
            .values(add_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        // First check if mission exists and belongs to this user
        let chief_id = edit_mission_entity.chief_id;
        let result = update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::chief_id.eq(chief_id))
            .filter(missions::deleted_at.is_null())
            .set(edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)
            .optional()?;

        match result {
            Some(id) => Ok(id),
            None => Err(anyhow::anyhow!(
                "Mission not found or you don't have permission to edit it"
            )),
        }
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let affected = update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::chief_id.eq(chief_id))
            .filter(missions::deleted_at.is_null())
            .set(missions::deleted_at.eq(now))
            .execute(&mut conn)?;

        if affected == 0 {
            return Err(anyhow::anyhow!(
                "Mission not found or you don't have permission to delete it"
            ));
        }

        Ok(())
    }
}
