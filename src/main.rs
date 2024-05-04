#![allow(dead_code, unused_variables, unused_imports)]

mod auth;
mod db;
mod env;
mod error;
mod openapi;
mod user;
mod web;

use std::{panic, sync::Arc};

use axum::{response::Response, routing::get, Router};
use env::{host, port};
use error::Error;
use openidconnect::core::CoreClient;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{fmt::time::SystemTime, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{auth::utils::oidc::new_oidc_client, db::connect_db, openapi::ApiDoc};

type WebResult = Result<Response, Error>;

#[derive(Clone)]
struct AppState {
    pub db: PgPool,
    pub oidc: Arc<CoreClient>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("axum-oidc-example=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(SystemTime))
        .init();

    panic::set_hook(Box::new(|e| error!("\n\n PANIC: {e:#?} \n")));

    let db = connect_db().await;

    sqlx::migrate!().run(&db).await.expect("Migrations failed");

    let oidc = new_oidc_client().await;
    let oidc = Arc::new(oidc);

    let router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(hello())
        .merge(auth::routes())
        .merge(user::routes())
        .with_state(AppState { db, oidc });

    let host = host();
    let port = port();

    info!("Server started on http://{host}:{port}");

    axum::serve(
        TcpListener::bind(format!("0.0.0.0:{port}").to_string())
            .await
            .unwrap(),
        router,
    )
    .await
    .expect("Server crashed");
}

fn hello() -> Router<AppState> {
    async fn handler() -> &'static str {
        "Hello"
    }
    Router::new().route("/", get(handler))
}
