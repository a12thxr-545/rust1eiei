use crate::{
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_image::Base64Image, brawler_model::RegisterBrawlerModel,
            uploaded_image::UploadedImage,
        },
    },
    infrastructure::{
        argon2::hash,
        cloudinary::{ UploadImageOptions},
        jwt::jwt_model::Passport,
    },
};
use anyhow::Result;
use std::sync::Arc;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn register(&self, mut register_model: RegisterBrawlerModel) -> Result<Passport> {
        let hashed_password = hash(register_model.password.clone())?;

        register_model.password = hashed_password;

        let register_entity = register_model.to_entity();

        let brawler_id = self.brawler_repository.register(register_entity).await?;

        let passport = Passport::new(brawler_id);
        Ok(passport)
    }
    pub async fn upload_avatar(
        &self,
        base64_image: String,
        brawler_id: i32,
    ) -> Result<UploadedImage> {
        let option = UploadImageOptions {
            folder: Some("brawlers_avatar".to_string()),
            public_id: Some(brawler_id.to_string()),
            transformation: Some("c_scale,w_256".to_string()),
        };

        let base64_image = Base64Image::new(&base64_image)?;

        let uploaded_image = self
            .brawler_repository
            .upload_avatar(brawler_id, base64_image, option)
            .await?;

        Ok(uploaded_image)
    }
}
