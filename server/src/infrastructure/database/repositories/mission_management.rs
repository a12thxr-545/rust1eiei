use crate::{
    domain::{
        entities::missions::{AddMissionEntity, EditMissionEntity},
        repositories::mission_management::MissionManagementRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{crew_memberships, missions},
    },
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, RunQueryDsl, dsl::now, dsl::update,
    insert_into,
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
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<i32> {
            let mut conn = db_pool.get()?;
            let res = insert_into(missions::table)
                .values(add_mission_entity)
                .returning(missions::id)
                .get_result::<i32>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<i32> {
            let mut conn = db_pool.get()?;

            // First check if mission exists and belongs to this user
            let chief_id = edit_mission_entity.chief_id;
            let res = update(missions::table)
                .filter(missions::id.eq(mission_id))
                .filter(missions::chief_id.eq(chief_id))
                .filter(missions::deleted_at.is_null())
                .set(edit_mission_entity)
                .returning(missions::id)
                .get_result::<i32>(&mut conn)
                .optional()?;

            match res {
                Some(id) => Ok(id),
                None => Err(anyhow::anyhow!(
                    "Mission not found or you don't have permission to edit it"
                )),
            }
        })
        .await??;

        Ok(result)
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;

            // Use a transaction to ensure both operations succeed or fail together
            conn.transaction::<(), anyhow::Error, _>(|c| {
                // First delete memberships (hard delete)
                diesel::delete(crew_memberships::table)
                    .filter(crew_memberships::mission_id.eq(mission_id))
                    .execute(c)?;

                // Then soft-delete the mission
                let affected = update(missions::table)
                    .filter(missions::id.eq(mission_id))
                    .filter(missions::chief_id.eq(chief_id))
                    .filter(missions::deleted_at.is_null())
                    .set(missions::deleted_at.eq(now))
                    .execute(c)?;

                if affected == 0 {
                    return Err(anyhow::anyhow!(
                        "Mission not found or you don't have permission to delete it"
                    ));
                }

                Ok(())
            })?;

            Ok(())
        })
        .await??;

        Ok(())
    }
}
