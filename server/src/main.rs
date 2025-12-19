use std::sync::Arc;

use server::{
    config::config_loader,
    infrastructure::{database::postgresql_connection, http::http_serv::start},
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!(".ENV LOADED");

    let postgres_pool = match postgresql_connection::establish_connection(&dotenvy_env.database.url)
    {
        Ok(pool) => pool,
        Err(err) => {
            error!("Fail to connect: {}", err);
            std::process::exit(1)
        }
    };
    info!("Connected DB");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server");
}
