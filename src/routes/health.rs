use axum::{extract::State, response::Json};
use serde::Serialize;

use crate::error::Result;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    database: String,
}

pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>> {
    let conn = state
        .db
        .connect()
        .map_err(crate::error::AppError::Database)?;
    conn.query("SELECT 1", ())
        .await
        .map_err(crate::error::AppError::Database)?;

    Ok(Json(HealthResponse {
        status: "healthy".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        database: "connected".into(),
    }))
}
