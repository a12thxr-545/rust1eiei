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
}
