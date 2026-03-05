use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::{
        database::postgresql_connection::PgPoolSquad, http::routers, realtime::RealtimeHub,
    },
};

fn static_serve() -> Router {
    let dir = "statics";
    let service = ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/index.html")));
    Router::new().fallback_service(service)
}

fn api_serve(db_pool: Arc<PgPoolSquad>, realtime_hub: Arc<RealtimeHub>) -> Router {
    Router::new()
        .nest("/brawlers", routers::brawlers::routes(Arc::clone(&db_pool)))
        .nest(
            "/authentication",
            routers::authentication::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/mission-management",
            routers::mission_management::routes(Arc::clone(&db_pool), Arc::clone(&realtime_hub)),
        )
        .nest(
            "/crew",
            routers::crew_operation::routes(Arc::clone(&db_pool), Arc::clone(&realtime_hub)),
        )
        .nest(
            "/mission",
            routers::mission_operation::routes(Arc::clone(&db_pool), Arc::clone(&realtime_hub)),
        )
        .nest(
            "/view",
            routers::mission_viewing::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/social",
            routers::social::routes(Arc::clone(&db_pool), Arc::clone(&realtime_hub)),
        )
        .nest("/rating", routers::rating::routes(Arc::clone(&db_pool)))
        .fallback(|| async { (StatusCode::NOT_FOUND, "API not found") })
}

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let realtime_hub = Arc::new(RealtimeHub::new());

    let app = Router::new()
        .route("/", get(|| async { "Backend is alive!" }))
        .nest("/api", api_serve(Arc::clone(&db_pool), realtime_hub))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Bind to [::] to support both IPv4 and IPv6 (Dual-stack)
    // This is more robust for cloud proxies that might try to connect via IPv6
    let addr = SocketAddr::new(
        std::net::IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED),
        config.server.port,
    );

    let listener = TcpListener::bind(addr).await?;

    info!("✅ SERVER READY AND WAITING FOR CONNECTIONS");
    info!("Address: {}", addr);
    info!("Railway Port Var: {:?}", std::env::var("PORT").ok());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Fail ctrl + c") };
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Receive ctrl + c signal"),
        _ = terminate => info!("Receive terminate signal"),
    }
}
