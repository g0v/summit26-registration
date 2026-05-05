use std::sync::Arc;

use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State, ws::WebSocketUpgrade},
    response::IntoResponse,
};

use crate::models::{
    Attendee, QrCodeDataRequest, RegistrationUpdate, VpDeeplinkQuery, VpDeeplinkResponse,
};

use super::{db, error::AppError, state::AppState, websocket::handle_registration_socket};

pub async fn verifier_callback(body: Bytes) -> Bytes {
    match std::str::from_utf8(&body) {
        Ok(text) => println!("{text}"),
        Err(_) => println!("{body:?}"),
    }

    body
}

pub async fn get_vp_deeplink(
    State(state): State<Arc<AppState>>,
    path: Option<Path<String>>,
    Query(query): Query<VpDeeplinkQuery>,
) -> Result<Json<VpDeeplinkResponse>, AppError> {
    let vp_uid = path
        .map(|Path(vp_uid)| vp_uid)
        .or(query.vp_uid)
        .filter(|vp_uid| !vp_uid.is_empty())
        .ok_or_else(|| AppError::bad_request("vpUid is required in path or query string"))?;

    let external_request = QrCodeDataRequest::for_vp_uid(vp_uid);
    let external_response = state
        .verifier_api
        .create_qrcode_data(&external_request)
        .await?;

    Ok(Json(VpDeeplinkResponse::fixed(external_response.auth_uri)))
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
