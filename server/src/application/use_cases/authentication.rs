use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::{
    // config::config_loader::get_jwt_env,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        self,
        jwt::{
            authentication_model::LoginModel,
            jwt_model::Passport,
        },
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
        let hsah_password =   brawler_entity.password;
        let login_password = login_model.password;

        if !infrastructure::argon2::verify(login_password, hsah_password)? {
            return Err(anyhow::anyhow!("Invalid username or password"));
        }

        let passport = Passport::new(brawler_entity.id);

        // let access_token_claims = Claims {
        //     sub: brawler.id.to_string(),
        //     exp: expires_in,
        //     iat: Utc::now().timestamp() as usize,
        // };

        // let access_token =
        //     infrastructure::jwt::generate_token(secret_env.secret, &access_token_claims)?;

        // let refresh_token_claims = Claims {
        //     sub: brawler.id.to_string(),
        //     exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        //     iat: Utc::now().timestamp() as usize,
        // };

        // let refresh_token =
        //     infrastructure::jwt::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;
        Ok(passport)
    }

    // pub async fn register(&self, refresh_token: String) -> Result<Passport> {
    //     let secret_env = get_jwt_env()?;
    //     let token_type = "Bearer".to_string();
    //     let expires_in = (Utc::now() + Duration::days(1)).timestamp() as usize;

    //     let claims =
    //         infrastructure::jwt::verify_token(secret_env.refresh_secret, refresh_token)?;

    //     let access_token_claims = Claims {
    //         sub: claims.sub.clone(),
    //         exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    //         iat: Utc::now().timestamp() as usize,
    //     };

    //     let access_token =
    //         infrastructure::jwt::generate_token(secret_env.secret, &access_token_claims)?;

    //     // let refresh_token_claims = Claims {
    //     //     sub: claims.sub,
    //     //     exp: claims.exp,
    //     //     iat: Utc::now().timestamp() as usize,
    //     // };

    //     // let refresh_token =
    //     //     infrastructure::jwt::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

    //     Ok(Passport {
    //         token_type,
    //         access_token,
    //         expires_in,
    //     })
    // }

//     pub async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
//         let secret_env = get_jwt_env()?;

//         let claims = verify_token(secret_env.refresh_secret, refresh_token.clone())?;

//         let access_token_claims = Claims {
//             sub: claims.sub.clone(),
//             exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
//             iat: Utc::now().timestamp() as usize,
//         };

//         let refresh_token_claims = Claims {
//             sub: claims.sub,
//             exp: claims.exp,
//             iat: Utc::now().timestamp() as usize,
//         };

//         let access_token = generate_token(secret_env.secret, &access_token_claims)?;

//         let refresh_token = generate_token(secret_env.life_time_days.to_string().clone(), &refresh_token_claims)?;

//         Ok(Passport {
//             token_type: refresh_token,
//             access_token: access_token,
//             expires_in: refresh_token_claims.exp,
//         })
//     }
}
