use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::time::sleep;
use tracing::error;

use crate::env::database_url;

pub async fn connect_db() -> PgPool {
    loop {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url())
            .await
        {
            Ok(db) => break db,
            Err(error) => {
                error!("Database connection error: {error:#?}");
                sleep(Duration::from_secs(5)).await
            }
        }
    }
}
