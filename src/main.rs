mod app;
mod config;
mod db;
mod error;
mod routes;
mod state;

use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, signal};
use tracing::info;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    config::init_config().expect("failed to load config");
    let cfg = config::get_config();

    let db = db::init_db().await;

    db::migrate(&db).await.expect("migration failed");

    let state = AppState { db: Arc::new(db) };

    let app = app::create_app(state);

    let addr: SocketAddr = format!("{}:{}", cfg.server.host, cfg.server.port)
        .parse()
        .expect("invalid address");

    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP Listener");

    info!("API listening on {}", addr);

    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    server.await.expect("server failed");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
            _ = ctrl_c => tracing::info!("Received Ctrl+C, starting graceful shutdown"),
            _ = terminate => {
                tracing::info!("Received SIGTERM, starting graceful shutdown");
            }
        }

    tokio::time::sleep(Duration::from_secs(10)).await;
    tracing::info!("Graceful shutdown complete");
}
