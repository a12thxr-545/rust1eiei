use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{config::config_loader::get_jwt_env, infrastructure::jwt::generate_token};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub id: i32,
    pub token_type: String,
    pub access_token: String,
    pub expires_in: usize,
    pub display_name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

impl Passport {
    pub fn new(
        brawler_id: i32,
        display_name: String,
        username: String,
        avatar_url: Option<String>,
        cover_url: Option<String>,
    ) -> Self {
        let jwt_env = get_jwt_env().unwrap();
        let token_type = "Bearer".to_string();
        let expires_in = (Utc::now() + Duration::days(jwt_env.life_time_days)).timestamp() as usize;

        let access_token_claims = Claims {
            sub: brawler_id,
            exp: expires_in,
            iat: Utc::now().timestamp() as usize,
            display_name: display_name.clone(),
            avatar_url: avatar_url.clone(),
        };

        let access_token = generate_token(jwt_env.secret, &access_token_claims).unwrap();

        Passport {
            id: brawler_id,
            token_type,
            access_token,
            expires_in,
            display_name,
            username,
            avatar_url,
            cover_url,
        }
    }
}
