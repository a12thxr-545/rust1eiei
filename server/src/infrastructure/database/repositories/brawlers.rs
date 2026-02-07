use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper, insert_into,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
        value_objects::{base64_image::Base64Image, uploaded_image::UploadedImage},
    },
    infrastructure::{
        cloudinary::UploadImageOptions,
        database::{postgresql_connection::PgPoolSquad, schema::brawlers},
    },
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
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let user_id = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(user_id)
    }

    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity> {
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
    async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage> {
        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::avatar_url.eq(uploaded_image.url.clone()),
                brawlers::avatar_public_id.eq(uploaded_image.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_image)
    }

    async fn upload_cover(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage> {
        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::cover_url.eq(uploaded_image.url.clone()),
                brawlers::cover_public_id.eq(uploaded_image.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_image)
    }

    async fn search(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<BrawlerEntity>, i64)> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        let offset = (page - 1) * page_size;

        let search_pattern = format!("%{}%", query);

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
}
