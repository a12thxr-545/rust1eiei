use anyhow::Result;

use crate::config::{
    config_model::{CloudinaryEnv, Database, DotEnvyConfig, JwtEnv, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("PORT")
            .or_else(|_| std::env::var("SERVER_PORT"))
            .unwrap_or_else(|_| "80".to_string())
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .unwrap_or_else(|_| "10485760".to_string())
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").unwrap_or_default().parse()?,
    };

    let secret = std::env::var("JWT_USER_SECRET")
        .unwrap_or_else(|_| "default_secret_for_railway".to_string())
        .parse()?;

    let refresh_secret = std::env::var("JWT_USER_REFRESH_SECRET")
        .unwrap_or_else(|_| "default_refresh_secret".to_string())
        .parse()?;

    let max_crew_per_mission = std::env::var("MAX_CREW_PER_MISSION")
        .unwrap_or_else(|_| "5".to_string())
        .parse()?;

    let config = DotEnvyConfig {
        server,
        database,
        secret,
        refresh_secret,
        max_crew_per_mission,
    };

    Ok(config)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_jwt_env() -> Result<JwtEnv> {
    dotenvy::dotenv().ok();

    let secret = std::env::var("JWT_USER_SECRET")
        .unwrap_or_else(|_| "default_secret_for_railway".to_string())
        .parse()?;

    let life_time_days = std::env::var("JWT_LIFE_TIME_DAYS")
        .unwrap_or_else(|_| "7".to_string())
        .parse::<i64>()?;

    Ok(JwtEnv {
        secret,
        life_time_days,
    })
}

pub fn get_cloundinary_env() -> Result<CloudinaryEnv> {
    dotenvy::dotenv().ok();

    let cloud_name =
        std::env::var("CLOUDINARY_CLOUD_NAME").unwrap_or_else(|_| "dtqrphm2b".to_string());

    let api_key =
        std::env::var("CLOUDINARY_API_KEY").unwrap_or_else(|_| "332441267638168".to_string());

    let api_secret = std::env::var("CLOUDINARY_API_SECRET")
        .unwrap_or_else(|_| "esTSyyi2tudHGwUnnX-zxdnOAxU".to_string());

    Ok(CloudinaryEnv {
        cloud_name,
        api_key,
        api_secret,
    })
}
