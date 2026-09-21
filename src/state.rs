use std::sync::Arc;
use libsql::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}
