use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::brawlers::{BrawlerEntity, NewBrawlerEntity};

#[async_trait]
pub trait BrawlerRepository {
    async fn find_by_id(&self, id: i32) -> Result<BrawlerEntity>;
    async fn find_by_username(&self, username: &str) -> Result<BrawlerEntity>;
    async fn register(&self, brawler: NewBrawlerEntity) -> Result<i32>;
    async fn update_avatar(
        &self,
        brawler_id: i32,
        avatar_url: String,
        avatar_public_id: String,
    ) -> Result<()>;
    async fn update_cover(
        &self,
        brawler_id: i32,
        cover_url: String,
        cover_public_id: String,
    ) -> Result<()>;
    async fn search(
        &self,
        query: Option<String>,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<BrawlerEntity>, i64)>;
    async fn update_display_name(&self, brawler_id: i32, display_name: String) -> Result<()>;
    async fn update_bio(&self, brawler_id: i32, bio: String) -> Result<()>;
}
