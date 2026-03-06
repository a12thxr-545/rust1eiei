use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    Router,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::{
        database::postgresql_connection::PgPoolSquad, http::routers, realtime::RealtimeHub,
    },
};

async fn request_logger(req: Request<axum::body::Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    info!("--> {} {}", method, path);

    let response = next.run(req).await;

    info!("<-- {} {} (Status: {})", method, path, response.status());

    response
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
        .layer(middleware::from_fn(request_logger))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "https://rust1eiei-cxoxu4uzr-a12thxr545s-projects.vercel.app"
                        .parse()
                        .unwrap(),
                    "https://rust1eiei-a12thxr545s-projects.vercel.app"
                        .parse()
                        .unwrap(),
                    "https://rust1eiei-8vnkk506c-a12thxr545s-projects.vercel.app"
                        .parse()
                        .unwrap(),
                    "https://rust1eiei-8y7kwoexb-a12thxr545s-projects.vercel.app"
                        .parse()
                        .unwrap(),
                    "https://rust1eiei-git-main-a12thxr545s-projects.vercel.app"
                        .parse()
                        .unwrap(),
                ])
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::ACCEPT,
                ])
                .allow_credentials(true),
        );

    // Bind to 0.0.0.0 to be safe, but use the port from config
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("🚀 SERVER STARTING...");
    info!("Binding to: {}", addr);
    info!("Railway Environment PORT: {:?}", std::env::var("PORT").ok());

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
