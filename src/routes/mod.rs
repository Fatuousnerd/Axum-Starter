use axum::{Router, routing::get};

mod health;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health::health))
}
