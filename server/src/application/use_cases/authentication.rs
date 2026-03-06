use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::{
    // config::config_loader::get_jwt_env,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        self,
        jwt::{authentication_model::LoginModel, jwt_model::Passport},
    },
};

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        // let secret_env = get_jwt_env()?;
        // let token_type = "Bearer".to_string();
        // let expires_in = (Utc::now() + Duration::days(1)).timestamp() as usize;

        let username = login_model.username.clone();

        let brawler_entity = self.brawler_repository.find_by_username(&username).await?;
        let hsah_password = brawler_entity.password;
        let login_password = login_model.password;

        if !infrastructure::argon2::verify(login_password, hsah_password)? {
            return Err(anyhow::anyhow!("Invalid username or password"));
        }

        let (joined_count, completed_count) =
            self.brawler_repository.get_stats(brawler_entity.id).await?;

        let passport = Passport::new(
            brawler_entity.id,
            brawler_entity.display_name,
            brawler_entity.username,
            brawler_entity.avatar_url,
            brawler_entity.cover_url,
            brawler_entity.bio,
            joined_count,
            completed_count,
        );

        Ok(passport)
    }
    pub async fn line_login(&self, code: &str) -> Result<Passport> {
        // Load line config
        let line_env = crate::config::config_loader::get_line_env()?;

        // Get access token
        let token_res = infrastructure::line::get_access_token(
            code,
            &line_env.channel_id,
            &line_env.channel_secret,
            &line_env.callback_url,
        )
        .await?;

        // Get profile
        let profile = infrastructure::line::get_profile(&token_res.access_token).await?;

        // Try to find brawler by line username
        let username = format!("line_{}", profile.user_id);
        tracing::info!("LINE login attempt for username: {}", username);

        let brawler_entity = match self.brawler_repository.find_by_username(&username).await {
            std::result::Result::Ok(existing) => {
                tracing::info!("Found existing LINE user: {}", existing.id);
                existing
            }
            std::result::Result::Err(e) => {
                tracing::info!(
                    "Brawler not found by username {} or other error: {:?}. Attempting registration.",
                    username,
                    e
                );

                // Not found, register new brawler
                let random_password = format!("line_pwd_{}", uuid::Uuid::new_v4());
                let hashed_password = infrastructure::argon2::hash(random_password)?;

                let new_brawler = crate::domain::entities::brawlers::NewBrawlerEntity {
                    username: username.clone(),
                    password: hashed_password,
                    display_name: profile.display_name.clone(),
                };

                match self.brawler_repository.register(new_brawler).await {
                    std::result::Result::Ok(brawler_id) => {
                        tracing::info!("Successfully registered new LINE user: {}", brawler_id);
                        // Update avatar if provided
                        if let Some(picture_url) = profile.picture_url {
                            let _ = self
                                .brawler_repository
                                .update_avatar(brawler_id, picture_url, "".to_string())
                                .await;
                        }

                        // Fetch the newly created entity
                        self.brawler_repository.find_by_username(&username).await?
                    }
                    std::result::Result::Err(reg_err) => {
                        let err_msg = reg_err.to_string();
                        if err_msg.contains("unique_username") {
                            tracing::warn!(
                                "Conflict for username {} during LINE registration. Row likely exists. Retrying find.",
                                username
                            );
                            self.brawler_repository.find_by_username(&username).await?
                        } else {
                            return Err(reg_err);
                        }
                    }
                }
            }
        };

        let (joined_count, completed_count) =
            self.brawler_repository.get_stats(brawler_entity.id).await?;

        let passport = Passport::new(
            brawler_entity.id,
            brawler_entity.display_name,
            brawler_entity.username,
            brawler_entity.avatar_url,
            brawler_entity.cover_url,
            brawler_entity.bio,
            joined_count,
            completed_count,
        );

        Ok(passport)
    }

    pub async fn get_me(&self, brawler_id: i32) -> Result<Passport> {
        let brawler_entity = self.brawler_repository.find_by_id(brawler_id).await?;
        let (joined_count, completed_count) =
            self.brawler_repository.get_stats(brawler_entity.id).await?;

        let passport = Passport::new(
            brawler_entity.id,
            brawler_entity.display_name,
            brawler_entity.username,
            brawler_entity.avatar_url,
            brawler_entity.cover_url,
            brawler_entity.bio,
            joined_count,
            completed_count,
        );

        Ok(passport)
    }
}
