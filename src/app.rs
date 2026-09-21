use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::routes;
use crate::state::AppState;

pub fn create_app() -> Router {
    let state = AppState::new();

    Router::new()
        .merge(routes::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
