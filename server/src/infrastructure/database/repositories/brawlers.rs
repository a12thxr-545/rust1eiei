use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper, insert_into,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, NewBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::brawlers},
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: NewBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let user_id = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(user_id)
    }

    async fn find_by_username(&self, username: &str) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::id.eq(brawler_id))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn update_avatar(
        &self,
        brawler_id: i32,
        avatar_url: String,
        avatar_public_id: String,
    ) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::avatar_url.eq(avatar_url),
                brawlers::avatar_public_id.eq(avatar_public_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn update_cover(
        &self,
        brawler_id: i32,
        cover_url: String,
        cover_public_id: String,
    ) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::cover_url.eq(cover_url),
                brawlers::cover_public_id.eq(cover_public_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn search(
        &self,
        query: Option<String>,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<BrawlerEntity>, i64)> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        let offset = (page - 1) * page_size;

        let search_pattern = format!("%{}%", query.unwrap_or_default());

        let total_count = brawlers::table
            .filter(
                brawlers::username
                    .ilike(search_pattern.clone())
                    .or(brawlers::display_name.ilike(search_pattern.clone())),
            )
            .count()
            .get_result::<i64>(&mut connection)
            .unwrap_or(0);

        let items = brawlers::table
            .filter(
                brawlers::username
                    .ilike(search_pattern.clone())
                    .or(brawlers::display_name.ilike(search_pattern)),
            )
            .select(BrawlerEntity::as_select())
            .limit(page_size)
            .offset(offset)
            .load::<BrawlerEntity>(&mut connection)?;

        Ok((items, total_count))
    }

    async fn update_display_name(&self, brawler_id: i32, display_name: String) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set(brawlers::display_name.eq(display_name))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn update_bio(&self, brawler_id: i32, bio: String) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set(brawlers::bio.eq(bio))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn get_stats(&self, brawler_id: i32) -> Result<(i64, i64)> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        use crate::infrastructure::database::schema::{crew_memberships, missions};

        let joined_count = crew_memberships::table
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .count()
            .get_result::<i64>(&mut connection)?;

        let completed_count = crew_memberships::table
            .inner_join(missions::table)
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .filter(missions::status.eq("Completed"))
            .count()
            .get_result::<i64>(&mut connection)?;

        Ok((joined_count, completed_count))
    }
}
