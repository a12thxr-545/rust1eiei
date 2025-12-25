use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{config::config_loader::get_jwt_env, infrastructure::jwt::generate_token};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: usize,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
}


impl Passport {
    pub fn new(brawler_id:i32) -> Self {
        let jwt_env = get_jwt_env().unwrap();
        let token_type = "Bearer".to_string();
        let expires_in = (Utc::now() + Duration::days(jwt_env.life_time_days)).timestamp() as usize;
        let display_name = format!("Brawler{}", brawler_id);
        let avatar_url = None;

        let access_token_claims = Claims {
            sub: brawler_id,
            exp: expires_in,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = generate_token(jwt_env.secret, &access_token_claims).unwrap();

        Passport {
            token_type,
            access_token,
            expires_in,
            display_name,
            avatar_url,
        }
    }
}   