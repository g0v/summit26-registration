use std::sync::Arc;

use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State, ws::WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
};

use crate::models::{
    Attendee, CallbackResponse, QrCodeDataRequest, RegistrationEvent, RegistrationTable,
    RegistrationUpdate, VerifierCallbackRequest, VpDeeplinkQuery, VpDeeplinkResponse, Worker,
};

use super::{db, error::AppError, state::AppState, websocket::handle_registration_socket};

pub async fn verifier_callback(
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> Result<Json<CallbackResponse>, AppError> {
    let body_text = String::from_utf8_lossy(&body);
    let Ok(callback) = serde_json::from_slice::<VerifierCallbackRequest>(&body) else {
        warn_invalid_callback(
            "callback body is not a valid verifier success payload",
            &body_text,
        );
        return Ok(Json(CallbackResponse::fail()));
    };

    let Some(ticket_id) = verified_ticket_id(&callback) else {
        warn_invalid_callback(
            "callback body did not contain a verified ticket_id",
            &body_text,
        );
        return Ok(Json(CallbackResponse::fail()));
    };

    let updated = db::register_ticket_id(&state.db, &ticket_id).await?;
    if updated.is_empty() {
        let message = format!("verifier callback ticket_id was not found: {ticket_id}");
        tracing::warn!("{message}");
        println!("WARNING: {message}");
        return Ok(Json(CallbackResponse::fail()));
    }

    for (table, update) in updated {
        let _ = state
            .registrations
            .send(RegistrationEvent::new(table, update));
    }

    Ok(Json(CallbackResponse::success()))
}

fn verified_ticket_id(callback: &VerifierCallbackRequest) -> Option<String> {
    if !callback.verify_result || callback.data.len() != 1 {
        return None;
    }

    callback.data[0]
        .claims
        .iter()
        .find(|claim| claim.ename == "ticket_id")
        .map(|claim| claim.value.clone())
        .filter(|ticket_id| !ticket_id.is_empty())
}

fn warn_invalid_callback(reason: &str, body: &str) {
    tracing::warn!(reason, body, "invalid verifier callback");
    println!("WARNING: {reason}: {body}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_example_contains_verified_ticket_id() {
        let callback = serde_json::from_str::<VerifierCallbackRequest>(include_str!(
            "../../callback.example.json"
        ))
        .expect("callback example should match required verifier callback fields");

        assert_eq!(verified_ticket_id(&callback).as_deref(), Some("G4925426"));
    }

    #[test]
    fn callback_without_verified_result_is_rejected() {
        let callback = VerifierCallbackRequest {
            verify_result: false,
            data: vec![crate::models::VerifierCallbackData {
                claims: vec![crate::models::VerifierCallbackClaim {
                    ename: "ticket_id".to_string(),
                    value: "CONF-1027".to_string(),
                }],
            }],
        };

        assert_eq!(verified_ticket_id(&callback), None);
    }
}

pub async fn auth_check() -> StatusCode {
    StatusCode::NO_CONTENT
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

pub async fn list_workers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Worker>>, AppError> {
    Ok(Json(db::list_workers(&state.db).await?))
}

pub async fn update_attendee_registration(
    State(state): State<Arc<AppState>>,
    Path(ticket_id): Path<String>,
    Json(payload): Json<RegistrationUpdate>,
) -> Result<Json<RegistrationUpdate>, AppError> {
    update_registration(state, RegistrationTable::Attendees, ticket_id, payload).await
}

pub async fn update_worker_registration(
    State(state): State<Arc<AppState>>,
    Path(ticket_id): Path<String>,
    Json(payload): Json<RegistrationUpdate>,
) -> Result<Json<RegistrationUpdate>, AppError> {
    update_registration(state, RegistrationTable::Workers, ticket_id, payload).await
}

async fn update_registration(
    state: Arc<AppState>,
    table: RegistrationTable,
    ticket_id: String,
    payload: RegistrationUpdate,
) -> Result<Json<RegistrationUpdate>, AppError> {
    if ticket_id != payload.ticket_id {
        return Err(AppError::bad_request(
            "ticket id in path and body do not match",
        ));
    }

    let updated = db::update_registration(table, &state.db, &payload)
        .await?
        .ok_or_else(|| AppError::not_found("attendee not found"))?;

    let _ = state
        .registrations
        .send(RegistrationEvent::new(table, updated.clone()));
    Ok(Json(updated))
}

pub async fn registration_socket(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_registration_socket(socket, state))
}
