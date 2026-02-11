use crate::{
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_image::Base64Image,
            brawler_model::{
                BrawlerPaginationModel, BrawlerProfileModel, BrawlerSummaryModel, PaginationModel,
                RegisterBrawlerModel,
            },
            uploaded_image::UploadedImage,
        },
    },
    infrastructure::{argon2::hash, cloudinary::UploadImageOptions, jwt::jwt_model::Passport},
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
        if self
            .brawler_repository
            .find_by_username(&register_model.username)
            .await
            .is_ok()
        {
            return Err(anyhow::anyhow!("Username already taken"));
        }

        let hashed_password = hash(register_model.password.clone())?;

        register_model.password = hashed_password;

        let register_entity = register_model.to_entity();

        let brawler_id = self.brawler_repository.register(register_entity).await?;

        let passport = Passport::new(
            brawler_id,
            register_model.display_name.clone(),
            register_model.username.clone(),
            None,
            None,
            None,
        );
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

        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        self.brawler_repository
            .update_avatar(
                brawler_id,
                uploaded_image.url.clone(),
                uploaded_image.public_id.clone(),
            )
            .await?;

        Ok(uploaded_image)
    }

    pub async fn get_profile(&self, brawler_id: i32) -> Result<Passport> {
        let brawler_entity = self.brawler_repository.find_by_id(brawler_id).await?;

        let passport = Passport::new(
            brawler_entity.id,
            brawler_entity.display_name,
            brawler_entity.username,
            brawler_entity.avatar_url,
            brawler_entity.cover_url,
            brawler_entity.bio,
        );

        Ok(passport)
    }

    pub async fn get_profile_by_username(&self, username: String) -> Result<BrawlerProfileModel> {
        let brawler_entity = self.brawler_repository.find_by_username(&username).await?;

        Ok(BrawlerProfileModel {
            id: brawler_entity.id,
            username: brawler_entity.username,
            display_name: brawler_entity.display_name,
            avatar_url: brawler_entity.avatar_url,
            cover_url: brawler_entity.cover_url,
            bio: brawler_entity.bio,
        })
    }

    pub async fn upload_cover(
        &self,
        base64_image: String,
        brawler_id: i32,
    ) -> Result<UploadedImage> {
        let option = UploadImageOptions {
            folder: Some("brawlers_cover".to_string()),
            public_id: Some(format!("cover_{}", brawler_id)),
            transformation: Some("c_fill,w_800,h_300".to_string()),
        };

        let base64_image = Base64Image::new(&base64_image)?;

        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        self.brawler_repository
            .update_cover(
                brawler_id,
                uploaded_image.url.clone(),
                uploaded_image.public_id.clone(),
            )
            .await?;

        Ok(uploaded_image)
    }

    pub async fn upload_chat_image(
        &self,
        base64_image: String,
        brawler_id: i32,
    ) -> Result<UploadedImage> {
        let option = UploadImageOptions {
            folder: Some("chat_images".to_string()),
            public_id: Some(format!(
                "chat_{}_{}",
                brawler_id,
                chrono::Utc::now().timestamp_millis()
            )),
            transformation: Some("c_limit,w_800".to_string()),
        };

        let base64_image = Base64Image::new(&base64_image)?;

        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        Ok(uploaded_image)
    }

    pub async fn search(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
    ) -> Result<BrawlerPaginationModel> {
        let (entities, total) = self
            .brawler_repository
            .search(Some(query.to_string()), page, page_size)
            .await?;

        let items = entities
            .into_iter()
            .map(|e| BrawlerSummaryModel {
                id: e.id,
                username: e.username,
                display_name: e.display_name,
                avatar_url: e.avatar_url,
                bio: e.bio,
            })
            .collect();

        Ok(BrawlerPaginationModel {
            pagination: PaginationModel {
                current_page: page,
                page_size,
                length: total,
            },
            items,
        })
    }

    pub async fn check_username(&self, username: String) -> Result<bool> {
        let exists = self
            .brawler_repository
            .find_by_username(&username)
            .await
            .is_ok();
        Ok(!exists)
    }

    pub async fn update_display_name(
        &self,
        brawler_id: i32,
        display_name: String,
    ) -> Result<Passport> {
        // Validate display name
        if display_name.trim().is_empty() {
            return Err(anyhow::anyhow!("Display name cannot be empty"));
        }
        if display_name.len() > 50 {
            return Err(anyhow::anyhow!(
                "Display name is too long (max 50 characters)"
            ));
        }

        // Update in database
        self.brawler_repository
            .update_display_name(brawler_id, display_name)
            .await?;

        // Return updated passport
        self.get_profile(brawler_id).await
    }

    pub async fn update_bio(&self, brawler_id: i32, bio: String) -> Result<BrawlerProfileModel> {
        // Limit bio length
        if bio.len() > 500 {
            return Err(anyhow::anyhow!("Bio is too long (max 500 characters)"));
        }

        // Update in database
        self.brawler_repository.update_bio(brawler_id, bio).await?;

        // Return updated profile
        let username = self
            .brawler_repository
            .find_by_id(brawler_id)
            .await?
            .username;
        self.get_profile_by_username(username).await
    }
}
