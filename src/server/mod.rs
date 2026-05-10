mod auth;
mod config;
mod db;
mod error;
mod rest_client;
mod routes;
mod state;
mod websocket;

use anyhow::Context;
use axum::{Router, middleware};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

use crate::models::RegistrationEvent;

use auth::{require_auth, require_https};
use config::load_settings;
use rest_client::VerifierApiClient;
use routes::{
    auth_check, get_vp_deeplink, list_attendees, list_workers, registration_socket,
    update_attendee_registration, update_worker_registration, verifier_callback,
};
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

    let (registrations, _) = broadcast::channel::<RegistrationEvent>(128);
    let verifier_api = VerifierApiClient::new(settings.verifier_api.clone());
    let state = AppState {
        db,
        registrations,
        verifier_api,
        auth: settings.auth.clone(),
    }
    .shared();

    let protected_routes = Router::new()
        .route("/api/auth/check", axum::routing::get(auth_check))
        .route("/api/attendees", axum::routing::get(list_attendees))
        .route(
            "/api/attendees/{ticket_id}/registration",
            axum::routing::post(update_attendee_registration),
        )
        .route("/api/workers", axum::routing::get(list_workers))
        .route(
            "/api/workers/{ticket_id}/registration",
            axum::routing::post(update_worker_registration),
        )
        .route("/ws/registrations", axum::routing::get(registration_socket))
        .fallback_service(
            ServeDir::new(settings.server.dist_dir).append_index_html_on_directories(true),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let app = Router::new()
        .route(
            "/api/verifier/callback",
            axum::routing::post(verifier_callback),
        )
        .route(
            "/api/verifier/deeplink/vp",
            axum::routing::get(get_vp_deeplink),
        )
        .route(
            "/api/verifier/deeplink/vp/{vp_uid}",
            axum::routing::get(get_vp_deeplink),
        )
        .merge(protected_routes)
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(AllowOrigin::mirror_request())
                .allow_methods(AllowMethods::mirror_request())
                .allow_headers(AllowHeaders::mirror_request()),
        )
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(state.clone(), require_https))
        .with_state(state);

    let configured_addr = format!("{}:{}", settings.server.bind_host, settings.server.port);
    let listener = TcpListener::bind((settings.server.bind_host.as_str(), settings.server.port))
        .await
        .with_context(|| format!("failed to bind server address {configured_addr}"))?;
    let bound_addr = listener
        .local_addr()
        .context("failed to read bound server address")?;
    tracing::info!("listening on http://{configured_addr} ({bound_addr})");

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
