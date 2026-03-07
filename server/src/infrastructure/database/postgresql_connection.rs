use anyhow::Result;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("src/infrastructure/database/migrations");

pub fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(15)
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(manager)?;

    // Run migrations on pool creation
    let mut connection = pool.get()?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;

    Ok(pool)
}
