mod app;
mod routes;
mod state;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = app::create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 7865));

    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP Listener");

    info!("API listening on {}", addr);

    axum::serve(listener, app).await.expect("server failed");
}
