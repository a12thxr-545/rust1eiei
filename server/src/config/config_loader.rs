use anyhow::Result;
use tracing::error;

use crate::config::{
    config_model::{Database, DotEnvyConfig, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .unwrap_or_else(|_| "2097152".to_string())
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()?,
        max_crew_per_mission: std::env::var("MAX_CREW_PER_MISSION")
            .unwrap_or_else(|_| "5".to_string())
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string()),
    };

    let secret = std::env::var("JWT_USER_SECRET")
        .unwrap_or_else(|_| "default_secret_key_for_development".to_string());

    let config = DotEnvyConfig {
        server,
        database,
        secret,
    };

    Ok(config)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_user_secret() -> Result<String> {
    let dotenvy_env = match load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };
    Ok(dotenvy_env.secret)
}
