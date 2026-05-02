use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate};

#[cfg(target_arch = "wasm32")]
use super::config::{configured_backend_host, is_configured_backend_port};

#[cfg(target_arch = "wasm32")]
pub fn start_backend_sync(
    set_attendees: WriteSignal<Vec<Attendee>>,
    set_sync_status: WriteSignal<String>,
) {
    use gloo_net::http::Request;
    use wasm_bindgen::{JsCast, closure::Closure};
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{ErrorEvent, Event, MessageEvent, WebSocket};

    use super::data::apply_registration_update;

    let (database_loaded, set_database_loaded) = signal(false);
    let (websocket_open, set_websocket_open) = signal(false);

    Effect::new(
        move |_| match (database_loaded.get(), websocket_open.get()) {
            (true, true) => set_sync_status.set("Live Data".to_string()),
            (true, false) => set_sync_status.set("Database loaded".to_string()),
            _ => set_sync_status.set("Sample data".to_string()),
        },
    );

    spawn_local(async move {
        match Request::get(&api_url("/api/attendees")).send().await {
            Ok(response) if response.ok() => match response.json::<Vec<Attendee>>().await {
                Ok(rows) => {
                    set_attendees.set(rows);
                    set_database_loaded.set(true);
                }
                Err(_) => set_database_loaded.set(false),
            },
            _ => set_database_loaded.set(false),
        }
    });

    let Ok(socket) = WebSocket::new(&registration_ws_url()) else {
        return;
    };

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let Some(payload) = event.data().as_string() else {
            return;
        };

        if let Ok(update) = serde_json::from_str::<RegistrationUpdate>(&payload) {
            apply_registration_update(set_attendees, &update);
        }
    });
    socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let onopen = Closure::<dyn FnMut(Event)>::new(move |_| {
        set_websocket_open.set(true);
    });
    socket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    let onerror = Closure::<dyn FnMut(ErrorEvent)>::new(move |_| {
        set_websocket_open.set(false);
    });
    socket.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn start_backend_sync(
    _set_attendees: WriteSignal<Vec<Attendee>>,
    _set_sync_status: WriteSignal<String>,
) {
}

#[cfg(target_arch = "wasm32")]
pub fn send_registration_update(update: RegistrationUpdate) {
    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;

    spawn_local(async move {
        let url = api_url(&format!("/api/attendees/{}/registration", update.ticket_id));
        let Ok(request) = Request::post(&url).json(&update) else {
            return;
        };

        let _ = request.send().await;
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn send_registration_update(_update: RegistrationUpdate) {}

#[cfg(target_arch = "wasm32")]
fn registration_ws_url() -> String {
    let Some(window) = web_sys::window() else {
        return format!("ws://{}/ws/registrations", configured_backend_host(None));
    };
    let location = window.location();
    let protocol = if location.protocol().ok().as_deref() == Some("https:") {
        "wss"
    } else {
        "ws"
    };
    let host = backend_host(&location);

    format!("{protocol}://{host}/ws/registrations")
}

#[cfg(target_arch = "wasm32")]
fn api_url(path: &str) -> String {
    let Some(window) = web_sys::window() else {
        return format!("http://{}{path}", configured_backend_host(None));
    };
    let location = window.location();
    let Ok(port) = location.port() else {
        return path.to_string();
    };

    if is_dev_frontend_port(&port) {
        let protocol = location.protocol().unwrap_or_else(|_| "http:".to_string());
        format!("{protocol}//{}{path}", backend_host(&location))
    } else {
        path.to_string()
    }
}

#[cfg(target_arch = "wasm32")]
fn backend_host(location: &web_sys::Location) -> String {
    match location.port().ok() {
        Some(port) if is_dev_frontend_port(&port) => configured_backend_host(Some(location)),
        _ => location
            .host()
            .unwrap_or_else(|_| configured_backend_host(Some(location))),
    }
}

#[cfg(target_arch = "wasm32")]
fn is_dev_frontend_port(port: &str) -> bool {
    !port.is_empty() && !is_configured_backend_port(port)
}
