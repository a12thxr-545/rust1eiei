use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    domain::{
        entities::missions::MissionEntity,
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::{brawler_model::BrawlerModel, mission_filter::MissionFilter},
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{brawlers, crew_memberships, missions},
    },
};
pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn crew_counting(&self, mission_id: i32) -> Result<i64> {
        let db_pool = Arc::clone(&self.db_pool);
        let count = tokio::task::spawn_blocking(move || -> Result<i64> {
            let mut conn = db_pool.get()?;
            let value = crew_memberships::table
                .filter(crew_memberships::mission_id.eq(mission_id))
                .count()
                .first::<i64>(&mut conn)?;
            Ok(value)
        })
        .await??;
        Ok(count)
    }

    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<MissionEntity> {
            let mut conn = db_pool.get()?;
            let res = missions::table
                .filter(missions::id.eq(mission_id))
                .filter(missions::deleted_at.is_null())
                .select(MissionEntity::as_select())
                .first::<MissionEntity>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn get_all(&self, mission_filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
        let db_pool = Arc::clone(&self.db_pool);
        let filter = mission_filter.clone();
        let value = tokio::task::spawn_blocking(move || -> Result<Vec<MissionEntity>> {
            let mut conn = db_pool.get()?;
            let mut query = missions::table
                .filter(missions::deleted_at.is_null())
                .into_boxed();

            if let Some(status) = &filter.status {
                query = query.filter(missions::status.eq(status));
            };
            if let Some(name) = &filter.name {
                query = query.filter(missions::name.ilike(format!("%{}%", name)));
            };
            if let Some(code) = &filter.code {
                query = query.filter(missions::code.eq(code.to_uppercase()));
            };
            if let Some(chief_id) = &filter.chief_id {
                query = query.filter(missions::chief_id.eq(*chief_id));
            };
            if let Some(exclude_chief_id) = &filter.exclude_chief_id {
                query = query.filter(missions::chief_id.ne(*exclude_chief_id));
            };

            if let Some(member_id) = &filter.member_id {
                let mission_ids = crew_memberships::table
                    .filter(crew_memberships::brawler_id.eq(*member_id))
                    .select(crew_memberships::mission_id)
                    .load::<i32>(&mut conn)?;
                query = query.filter(missions::id.eq_any(mission_ids));
            }

            if let Some(exclude_member_id) = &filter.exclude_member_id {
                let mission_ids = crew_memberships::table
                    .filter(crew_memberships::brawler_id.eq(*exclude_member_id))
                    .select(crew_memberships::mission_id)
                    .load::<i32>(&mut conn)?;
                query = query.filter(diesel::dsl::not(missions::id.eq_any(mission_ids)));
            }

            let value = query
                .select(MissionEntity::as_select())
                .order_by(missions::created_at.desc())
                .load::<MissionEntity>(&mut conn)?;

            Ok(value)
        })
        .await??;

        Ok(value)
    }
    async fn get_mission_count(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<BrawlerModel>> {
            let mut conn = db_pool.get()?;
            let sql = r#"
            SELECT 
                b.id AS brawler_id,
                b.display_name,
                b.username,
                COALESCE(b.avatar_url, '') AS avatar_url,
                b.bio,
               COALESCE(s.success_count, 0) AS mission_success_count,
               COALESCE(j.joined_count, 0) AS mission_joined_count
            FROM 
                crew_memberships cm
            INNER JOIN 
                brawlers b ON b.id = cm.brawler_id 
            LEFT JOIN 
                (
                    SELECT 
                        cm2.brawler_id, 
                        COUNT(*) AS success_count
                    FROM 
                        crew_memberships cm2
                    INNER JOIN 
                        missions m2 ON m2.id = cm2.mission_id
                    WHERE 
                        m2.status = 'Completed'
                    GROUP BY 
                        cm2.brawler_id
                ) s ON s.brawler_id = cm.brawler_id
            LEFT JOIN 
                (
                    SELECT 
                        cm3.brawler_id, 
                        COUNT(*) AS joined_count
                    FROM 
                        crew_memberships cm3
                    GROUP BY 
                        cm3.brawler_id
                ) j ON j.brawler_id = b.id
            WHERE 
                cm.mission_id = $1
        "#;
            let res = diesel::sql_query(sql)
                .bind::<diesel::sql_types::Int4, _>(mission_id)
                .load::<BrawlerModel>(&mut conn)?;
            Ok(res)
        })
        .await??;

        Ok(result)
    }

    async fn get_chief_name(&self, chief_id: i32) -> Result<String> {
        let db_pool = Arc::clone(&self.db_pool);
        let name = tokio::task::spawn_blocking(move || -> Result<String> {
            let mut conn = db_pool.get()?;
            let name = brawlers::table
                .filter(brawlers::id.eq(chief_id))
                .select(brawlers::display_name)
                .first::<String>(&mut conn)?;
            Ok(name)
        })
        .await??;

        Ok(name)
    }
}
