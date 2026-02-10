use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::mission_invitations::{AddMissionInvitationEntity, MissionInvitationEntity},
        repositories::mission_invitations::MissionInvitationRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::mission_invitations},
};

pub struct MissionInvitationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionInvitationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionInvitationRepository for MissionInvitationPostgres {
    async fn invite(&self, entity: AddMissionInvitationEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = diesel::insert_into(mission_invitations::table)
            .values(&entity)
            .returning(mission_invitations::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn accept(&self, invitation_id: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;
            diesel::update(mission_invitations::table)
                .filter(mission_invitations::id.eq(invitation_id))
                .set(mission_invitations::status.eq("accepted"))
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn reject(&self, invitation_id: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;
            diesel::update(mission_invitations::table)
                .filter(mission_invitations::id.eq(invitation_id))
                .set(mission_invitations::status.eq("rejected"))
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn get_received_invitations(&self, uid: i32) -> Result<Vec<MissionInvitationEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = mission_invitations::table
            .filter(
                mission_invitations::invitee_id
                    .eq(uid)
                    .and(mission_invitations::status.eq("pending")),
            )
            .load::<MissionInvitationEntity>(&mut conn)?;
        Ok(result)
    }

    async fn get_mission_invitations(&self, mid: i32) -> Result<Vec<MissionInvitationEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = mission_invitations::table
            .filter(mission_invitations::mission_id.eq(mid))
            .load::<MissionInvitationEntity>(&mut conn)?;
        Ok(result)
    }

    async fn get_by_id(&self, invitation_id: i32) -> Result<MissionInvitationEntity> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<MissionInvitationEntity> {
            let mut conn = db_pool.get()?;
            let res = mission_invitations::table
                .filter(mission_invitations::id.eq(invitation_id))
                .first::<MissionInvitationEntity>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn delete_existing(&self, mid: i32, uid: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(mission_invitations::table)
            .filter(mission_invitations::mission_id.eq(mid))
            .filter(mission_invitations::invitee_id.eq(uid))
            .execute(&mut conn)?;
        Ok(())
    }
}
