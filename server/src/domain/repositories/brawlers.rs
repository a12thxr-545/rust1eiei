use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        value_objects::{base64_image::Base64Image, uploaded_image::UploadedImage},
    },
    infrastructure::cloudinary::UploadImageOptions,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity>;
    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity>;
    async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage>;
    async fn upload_cover(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage>;
    async fn search(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<BrawlerEntity>, i64)>;
    async fn update_display_name(&self, brawler_id: i32, display_name: String) -> Result<()>;
}
