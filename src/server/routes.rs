use std::sync::Arc;

use axum::{
    Json,
    body::Bytes,
    extract::{Path, State, ws::WebSocketUpgrade},
    response::IntoResponse,
};

use crate::models::{Attendee, RegistrationUpdate};

use super::{db, error::AppError, state::AppState, websocket::handle_registration_socket};

pub async fn echo_body(body: Bytes) -> Bytes {
    body
}

pub async fn list_attendees(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Attendee>>, AppError> {
    Ok(Json(db::list_attendees(&state.db).await?))
}

pub async fn update_registration(
    State(state): State<Arc<AppState>>,
    Path(ticket_id): Path<String>,
    Json(payload): Json<RegistrationUpdate>,
) -> Result<Json<RegistrationUpdate>, AppError> {
    if ticket_id != payload.ticket_id {
        return Err(AppError::bad_request(
            "ticket id in path and body do not match",
        ));
    }

    let updated = db::update_registration(&state.db, &payload)
        .await?
        .ok_or_else(|| AppError::not_found("attendee not found"))?;

    let _ = state.registrations.send(updated.clone());
    Ok(Json(updated))
}

pub async fn registration_socket(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_registration_socket(socket, state))
}
