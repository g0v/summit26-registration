use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate, Worker};

#[cfg(target_arch = "wasm32")]
use crate::models::{RegistrationEvent, RegistrationTable};

#[cfg(target_arch = "wasm32")]
use super::config::{api_url, websocket_url};

#[cfg(target_arch = "wasm32")]
pub fn start_backend_sync(
    set_attendees: WriteSignal<Vec<Attendee>>,
    set_workers: WriteSignal<Vec<Worker>>,
    set_sync_status: WriteSignal<String>,
    set_authenticated: WriteSignal<bool>,
) {
    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::RequestCredentials;

    let (database_loaded, set_database_loaded) = signal(false);
    let (websocket_open, set_websocket_open) = signal(false);

    Effect::new(
        move |_| match (database_loaded.get(), websocket_open.get()) {
            (true, true) => set_sync_status.set("Live Data".to_string()),
            (true, false) => set_sync_status.set("Database Loaded".to_string()),
            _ => set_sync_status.set("Sample Data".to_string()),
        },
    );

    spawn_local(async move {
        match Request::get(&api_url("/api/auth/check"))
            .credentials(RequestCredentials::Include)
            .send()
            .await
        {
            Ok(response) if response.ok() => {}
            _ => {
                set_authenticated.set(false);
                set_sync_status.set("Authentication Failed, Showing Sample Data".to_string());
                return;
            }
        }
        set_authenticated.set(true);

        match Request::get(&api_url("/api/attendees"))
            .credentials(RequestCredentials::Include)
            .send()
            .await
        {
            Ok(response) if response.ok() => match response.json::<Vec<Attendee>>().await {
                Ok(rows) => {
                    set_attendees.set(rows);
                    set_database_loaded.set(true);
                }
                Err(_) => set_database_loaded.set(false),
            },
            _ => set_database_loaded.set(false),
        }

        match Request::get(&api_url("/api/workers"))
            .credentials(RequestCredentials::Include)
            .send()
            .await
        {
            Ok(response) if response.ok() => match response.json::<Vec<Worker>>().await {
                Ok(rows) => set_workers.set(rows),
                Err(_) => set_database_loaded.set(false),
            },
            _ => set_database_loaded.set(false),
        }

        open_registration_socket(set_attendees, set_workers, set_websocket_open);
    });
}

#[cfg(target_arch = "wasm32")]
fn open_registration_socket(
    set_attendees: WriteSignal<Vec<Attendee>>,
    set_workers: WriteSignal<Vec<Worker>>,
    set_websocket_open: WriteSignal<bool>,
) {
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{ErrorEvent, Event, MessageEvent, WebSocket};

    use super::data::{apply_attendee_registration_update, apply_worker_registration_update};

    let Ok(socket) = WebSocket::new(&registration_ws_url()) else {
        return;
    };

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let Some(payload) = event.data().as_string() else {
            return;
        };

        if let Ok(event) = serde_json::from_str::<RegistrationEvent>(&payload) {
            let update = event.update();
            match event.table {
                RegistrationTable::Attendees => {
                    apply_attendee_registration_update(set_attendees, &update);
                }
                RegistrationTable::Workers => {
                    apply_worker_registration_update(set_workers, &update);
                }
            }
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
    _set_workers: WriteSignal<Vec<Worker>>,
    _set_sync_status: WriteSignal<String>,
    _set_authenticated: WriteSignal<bool>,
) {
}

#[cfg(target_arch = "wasm32")]
pub fn send_attendee_registration_update(update: RegistrationUpdate) {
    send_registration_update("/api/attendees", update);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn send_attendee_registration_update(_update: RegistrationUpdate) {}

#[cfg(target_arch = "wasm32")]
pub fn send_worker_registration_update(update: RegistrationUpdate) {
    send_registration_update("/api/workers", update);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn send_worker_registration_update(_update: RegistrationUpdate) {}

#[cfg(target_arch = "wasm32")]
fn send_registration_update(resource_path: &str, update: RegistrationUpdate) {
    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::RequestCredentials;

    let resource_path = resource_path.to_string();
    spawn_local(async move {
        let url = api_url(&format!(
            "{}/{}/registration",
            resource_path, update.ticket_id
        ));
        let Ok(request) = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&update)
        else {
            return;
        };

        let _ = request.send().await;
    });
}

#[cfg(target_arch = "wasm32")]
fn registration_ws_url() -> String {
    let location = web_sys::window().map(|window| window.location());
    websocket_url("/ws/registrations", location.as_ref())
}
