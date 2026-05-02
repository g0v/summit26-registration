mod config;
mod db;
mod error;
mod routes;
mod state;
mod websocket;

use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

use crate::models::RegistrationUpdate;

use config::load_settings;
use routes::{list_attendees, registration_socket, update_registration};
use state::AppState;

pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let settings = load_settings().context("failed to load settings")?;
    let db = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await
        .context("failed to connect to postgres")?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("failed to run migrations")?;

    let (registrations, _) = broadcast::channel::<RegistrationUpdate>(128);
    let state = AppState { db, registrations }.shared();

    let app = Router::new()
        .route("/api/attendees", axum::routing::get(list_attendees))
        .route(
            "/api/attendees/{ticket_id}/registration",
            axum::routing::post(update_registration),
        )
        .route("/ws/registrations", axum::routing::get(registration_socket))
        .fallback_service(
            ServeDir::new(settings.server.dist_dir).append_index_html_on_directories(true),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .context("invalid server address")?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "summit26_registration=debug,tower_http=info,axum=info".into()),
        )
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
