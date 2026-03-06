use std::sync::Arc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

    if dotenvy_env.database.url.is_empty() {
        error!("❌ DATABASE_URL is missing! Please set it in Railway Variables.");
        std::process::exit(1);
    }

    info!("Connecting to database...");
    let postgres_pool = match postgresql_connection::establish_connection(&dotenvy_env.database.url)
    {
        Ok(pool) => pool,
        Err(err) => {
            error!(
                "❌ Database connection failed: {}. Ensure your DATABASE_URL is correct.",
                err
            );
            std::process::exit(1)
        }
    };
    info!("Connected DB");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server");
}
