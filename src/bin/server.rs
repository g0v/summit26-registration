use std::{
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{
        Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use config::Config;
use futures::{SinkExt, StreamExt};
use summit26_registration::models::{Attendee, RegistrationUpdate};
use serde::Deserialize;
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

#[derive(Clone)]
struct AppState {
    db: PgPool,
    registrations: broadcast::Sender<RegistrationUpdate>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    server: ServerSettings,
    database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
struct ServerSettings {
    host: String,
    port: u16,
    dist_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct DatabaseSettings {
    url: String,
    max_connections: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "summit26_registration=debug,tower_http=info,axum=info".into()),
        )
        .init();

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

    let (registrations, _) = broadcast::channel(128);
    let state = AppState { db, registrations };

    let app = Router::new()
        .route("/api/attendees", get(list_attendees))
        .route(
            "/api/attendees/{ticket_id}/registration",
            post(update_registration),
        )
        .route("/ws/registrations", get(registration_socket))
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
        .with_state(Arc::new(state));

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

fn load_settings() -> Result<Settings, config::ConfigError> {
    let config_path = std::env::var("APP_CONFIG").unwrap_or_else(|_| {
        if FsPath::new("config.toml").exists() {
            "config.toml".to_string()
        } else {
            "config.example.toml".to_string()
        }
    });

    Config::builder()
        .add_source(config::File::with_name(&config_path))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?
        .try_deserialize()
}

async fn list_attendees(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Attendee>>, AppError> {
    let rows = sqlx::query(
        r#"
            SELECT name, ticket_id, ticket_type, registered
            FROM attendees
            ORDER BY name
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    let attendees = rows
        .into_iter()
        .map(|row| Attendee {
            name: row.get("name"),
            ticket_id: row.get("ticket_id"),
            ticket_type: row.get("ticket_type"),
            registered: row.get("registered"),
        })
        .collect();

    Ok(Json(attendees))
}

async fn update_registration(
    State(state): State<Arc<AppState>>,
    Path(ticket_id): Path<String>,
    Json(payload): Json<RegistrationUpdate>,
) -> Result<Json<RegistrationUpdate>, AppError> {
    if ticket_id != payload.ticket_id {
        return Err(AppError::bad_request(
            "ticket id in path and body do not match",
        ));
    }

    let updated = sqlx::query(
        r#"
            UPDATE attendees
            SET registered = $1, updated_at = NOW()
            WHERE ticket_id = $2
            RETURNING ticket_id, registered
        "#,
    )
    .bind(payload.registered)
    .bind(&payload.ticket_id)
    .fetch_optional(&state.db)
    .await?
    .map(|row| RegistrationUpdate {
        ticket_id: row.get("ticket_id"),
        registered: row.get("registered"),
    })
    .ok_or_else(|| AppError::not_found("attendee not found"))?;

    let _ = state.registrations.send(updated.clone());
    Ok(Json(updated))
}

async fn registration_socket(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_registration_socket(socket, state))
}

async fn handle_registration_socket(socket: WebSocket, state: Arc<AppState>) {
    let mut updates = state.registrations.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(update) = updates.recv().await {
            let Ok(message) = serde_json::to_string(&update) else {
                continue;
            };

            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = receiver.next().await {
        if matches!(message, Message::Close(_)) {
            break;
        }
    }

    send_task.abort();
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

#[derive(Debug)]
struct AppError {
    status: StatusCode,
    message: String,
}

impl AppError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!(?error, "database error");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "database error".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}
