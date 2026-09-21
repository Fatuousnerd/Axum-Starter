use libsql::{Builder, Database};

use crate::config::get_config;

pub async fn init_db() -> Database {
    let cfg = get_config();
    let db_cfg = &cfg.database;

    let db = if let Some(url) = &db_cfg.url {
        if !url.is_empty() {
            tracing::info!("Connecting to remote Turso DB at {}", url);
            let auth_token = db_cfg.auth_token.clone().unwrap_or_default();
            Builder::new_remote(url.clone(), auth_token)
                .build()
                .await
                .expect("failed to create remote db")
        } else {
            tracing::info!("Using local Turso DB at {}", db_cfg.local_path);
            Builder::new_local(&db_cfg.local_path)
                .build()
                .await
                .expect("failed to create local db")
        }
    } else {
        tracing::info!("Using local Turso DB at {}", db_cfg.local_path);
        Builder::new_local(&db_cfg.local_path)
            .build()
            .await
            .expect("failed to create local db")
    };

    db
}

pub async fn migrate(db: &Database) -> Result<(), libsql::Error> {
    let conn = db.connect()?;

    conn.execute(include_str!("../migrations/001_init.sql"), ())
        .await?;

    Ok(())
}
