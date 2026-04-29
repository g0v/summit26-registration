use leptos::mount::mount_to_body;

fn main() {
    mount_to_body(App);
}

use leptos::prelude::*;
use summit26_registration::models::{Attendee, RegistrationUpdate};

#[component]
fn App() -> impl IntoView {
    let (attendees, set_attendees) = signal(sample_attendees());
    let (sync_status, set_sync_status) = signal("Sample data".to_string());

    start_backend_sync(set_attendees, set_sync_status);

    view! {
        <style>
            {r#"
                :root {
                    color: #1d2630;
                    background: #f4f6f8;
                    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
                    line-height: 1.5;
                }

                * {
                    box-sizing: border-box;
                }

                body {
                    margin: 0;
                    min-width: 320px;
                    min-height: 100vh;
                    background:
                        linear-gradient(180deg, rgba(255, 255, 255, 0.72), rgba(244, 246, 248, 0.92)),
                        #f4f6f8;
                }

                button,
                input {
                    font: inherit;
                }

                .app-shell {
                    min-height: 100vh;
                    padding: 32px;
                }

                .topbar {
                    display: flex;
                    align-items: end;
                    justify-content: space-between;
                    gap: 24px;
                    max-width: 1120px;
                    margin: 0 auto 18px;
                }

                .eyebrow {
                    margin: 0 0 4px;
                    color: #607080;
                    font-size: 13px;
                    font-weight: 700;
                    letter-spacing: 0.08em;
                    text-transform: uppercase;
                }

                h1 {
                    margin: 0;
                    color: #101820;
                    font-size: clamp(28px, 5vw, 44px);
                    line-height: 1.05;
                    letter-spacing: 0;
                }

                .shift-status {
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    min-width: 168px;
                    padding: 10px 12px;
                    border: 1px solid #cfd8df;
                    border-radius: 8px;
                    background: #ffffff;
                    color: #344452;
                    font-size: 14px;
                    font-weight: 700;
                    box-shadow: 0 8px 24px rgba(30, 42, 54, 0.08);
                }

                .shift-status.is-live .status-dot {
                    background: #1b9a59;
                    box-shadow: 0 0 0 4px rgba(27, 154, 89, 0.14);
                }

                .shift-status.is-sample .status-dot {
                    background: #c77817;
                    box-shadow: 0 0 0 4px rgba(199, 120, 23, 0.14);
                }

                .status-dot {
                    width: 10px;
                    height: 10px;
                    border-radius: 999px;
                }

                .table-wrap {
                    max-width: 1120px;
                    margin: 0 auto;
                    overflow-x: auto;
                    border: 1px solid #cbd5dc;
                    border-radius: 8px;
                    background: #ffffff;
                    box-shadow: 0 18px 44px rgba(24, 36, 48, 0.12);
                }

                table {
                    width: 100%;
                    min-width: 760px;
                    border-collapse: collapse;
                }

                th,
                td {
                    height: 52px;
                    padding: 0 14px;
                    border-right: 1px solid #dbe2e7;
                    border-bottom: 1px solid #dbe2e7;
                    text-align: left;
                    white-space: nowrap;
                }

                th:last-child,
                td:last-child {
                    border-right: 0;
                }

                tbody tr:last-child td {
                    border-bottom: 0;
                }

                th {
                    position: sticky;
                    top: 0;
                    z-index: 1;
                    height: 44px;
                    background: #e8edf1;
                    color: #3f4f5d;
                    font-size: 12px;
                    font-weight: 800;
                    letter-spacing: 0.04em;
                    text-transform: uppercase;
                }

                tbody tr {
                    background: #ffffff;
                }

                tbody tr:nth-child(even) {
                    background: #f9fbfc;
                }

                tbody tr.is-registered {
                    background: #eef9f3;
                }

                .attendee-name {
                    color: #121a22;
                    font-weight: 800;
                }

                .ticket-id {
                    color: #44515e;
                    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
                    font-size: 13px;
                }

                .ticket-type {
                    display: inline-flex;
                    align-items: center;
                    min-width: 86px;
                    height: 28px;
                    padding: 0 10px;
                    border-radius: 999px;
                    background: #edf1f5;
                    color: #293845;
                    font-size: 13px;
                    font-weight: 800;
                }

                .check-cell {
                    width: 132px;
                    text-align: center;
                }

                .check-control {
                    display: inline-grid;
                    place-items: center;
                    width: 28px;
                    height: 28px;
                    border: 1px solid #aab8c4;
                    border-radius: 6px;
                    background: #ffffff;
                    cursor: pointer;
                }

                .check-control input {
                    width: 18px;
                    height: 18px;
                    margin: 0;
                    accent-color: #1b9a59;
                    cursor: pointer;
                }

                .registered-label {
                    color: #247246;
                    font-size: 13px;
                    font-weight: 800;
                }

                .pending-label {
                    color: #7a4b14;
                    font-size: 13px;
                    font-weight: 800;
                }

                @media (max-width: 720px) {
                    .app-shell {
                        padding: 18px;
                    }

                    .topbar {
                        align-items: stretch;
                        flex-direction: column;
                        gap: 14px;
                    }

                    .shift-status {
                        width: 100%;
                    }
                }
            "#}
        </style>

        <main class="app-shell">
            <header class="topbar">
                <div>
                    <p class="eyebrow">"Conference Registration"</p>
                    <h1>"Attendee Check-in"</h1>
                </div>
                <div
                    class=move || {
                        if sync_status.get().contains("Live") {
                            "shift-status is-live"
                        } else {
                            "shift-status is-sample"
                        }
                    }
                    aria-label="Registration desk status"
                >
                    <span class="status-dot" aria-hidden="true"></span>
                    {move || sync_status.get()}
                </div>
            </header>

            <section class="table-wrap" aria-label="Attendee registration spreadsheet">
                <table>
                    <thead>
                        <tr>
                            <th scope="col">"Name"</th>
                            <th scope="col">"Ticket ID"</th>
                            <th scope="col">"Ticket Type"</th>
                            <th scope="col" class="check-cell">"Registered"</th>
                            <th scope="col">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || {
                                attendees
                                    .get()
                                    .into_iter()
                                    .map(|attendee| attendee.ticket_id)
                                    .collect::<Vec<_>>()
                            }
                            key=|ticket_id| ticket_id.clone()
                            children=move |ticket_id| {
                                view! {
                                    <AttendeeRow
                                        ticket_id= ticket_id
                                        attendees=attendees
                                        on_toggle=Callback::new(move |update: RegistrationUpdate| {
                                            apply_registration_update(set_attendees, &update);
                                            send_registration_update(update);
                                        })
                                    />
                                }
                            }
                        />
                    </tbody>
                </table>
            </section>
        </main>
    }
}

#[component]
fn AttendeeRow(
    ticket_id: String,
    attendees: ReadSignal<Vec<Attendee>>,
    #[prop(into)] on_toggle: Callback<RegistrationUpdate>,
) -> impl IntoView {
    let row_ticket_id = ticket_id.clone();
    let name_cell_ticket_id = ticket_id.clone();
    let ticket_cell_ticket_id = ticket_id.clone();
    let type_cell_ticket_id = ticket_id.clone();
    let aria_ticket_id = ticket_id.clone();
    let checked_ticket_id = ticket_id.clone();
    let change_ticket_id = ticket_id.clone();
    let status_class_ticket_id = ticket_id.clone();
    let status_text_ticket_id = ticket_id.clone();

    view! {
        <tr class:is-registered=move || {
            attendee_by_ticket(attendees, &row_ticket_id)
                .is_some_and(|attendee| attendee.registered)
        }>
            <td class="attendee-name">
                {move || {
                    attendee_by_ticket(attendees, &name_cell_ticket_id)
                        .map(|attendee| attendee.name)
                        .unwrap_or_default()
                }}
            </td>
            <td class="ticket-id">{ticket_cell_ticket_id}</td>
            <td>
                <span class="ticket-type">
                    {move || {
                        attendee_by_ticket(attendees, &type_cell_ticket_id)
                            .map(|attendee| attendee.ticket_type)
                            .unwrap_or_default()
                    }}
                </span>
            </td>
            <td class="check-cell">
                <label
                    class="check-control"
                    aria-label=move || {
                        let name = attendee_by_ticket(attendees, &aria_ticket_id)
                            .map(|attendee| attendee.name)
                            .unwrap_or_else(|| aria_ticket_id.clone());
                        format!("Register {name}")
                    }
                >
                    <input
                        type="checkbox"
                        prop:checked=move || {
                            attendee_by_ticket(attendees, &checked_ticket_id)
                                .is_some_and(|attendee| attendee.registered)
                        }
                        on:change=move |event| {
                            on_toggle.run(RegistrationUpdate {
                                ticket_id: change_ticket_id.clone(),
                                registered: event_target_checked(&event),
                            });
                        }
                    />
                </label>
            </td>
            <td>
                <span class=move || if attendee_by_ticket(attendees, &status_class_ticket_id)
                    .is_some_and(|attendee| attendee.registered) {
                        "registered-label"
                    } else {
                        "pending-label"
                    }>
                    {move || if attendee_by_ticket(attendees, &status_text_ticket_id)
                        .is_some_and(|attendee| attendee.registered) {
                            "Registered"
                        } else {
                            "Pending"
                    }}
                </span>
            </td>
        </tr>
    }
}

fn attendee_by_ticket(attendees: ReadSignal<Vec<Attendee>>, ticket_id: &str) -> Option<Attendee> {
    attendees
        .get()
        .into_iter()
        .find(|attendee| attendee.ticket_id == ticket_id)
}

fn sample_attendees() -> Vec<Attendee> {
    [
        ("Maya Chen", "CONF-1027", "Speaker", true),
        ("Owen Patel", "CONF-1184", "VIP", false),
        ("Lina Morales", "CONF-1266", "General", false),
        ("Noah Williams", "CONF-1315", "Workshop", true),
        ("Ari Tanaka", "CONF-1442", "General", false),
        ("Sam Rivera", "CONF-1503", "Sponsor", false),
        ("Priya Shah", "CONF-1638", "VIP", true),
        ("Theo Brooks", "CONF-1790", "General", false),
    ]
    .into_iter()
    .map(|(name, ticket_id, ticket_type, registered)| Attendee {
        name: name.to_string(),
        ticket_id: ticket_id.to_string(),
        ticket_type: ticket_type.to_string(),
        registered,
    })
    .collect()
}

fn apply_registration_update(
    set_attendees: WriteSignal<Vec<Attendee>>,
    update: &RegistrationUpdate,
) {
    set_attendees.update(|attendees| {
        if let Some(attendee) = attendees
            .iter_mut()
            .find(|attendee| attendee.ticket_id == update.ticket_id)
        {
            attendee.registered = update.registered;
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn start_backend_sync(
    set_attendees: WriteSignal<Vec<Attendee>>,
    set_sync_status: WriteSignal<String>,
) {
    use gloo_net::http::Request;
    use wasm_bindgen::{JsCast, closure::Closure};
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{ErrorEvent, Event, MessageEvent, WebSocket};

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
fn start_backend_sync(
    _set_attendees: WriteSignal<Vec<Attendee>>,
    _set_sync_status: WriteSignal<String>,
) {
}

#[cfg(target_arch = "wasm32")]
fn send_registration_update(update: RegistrationUpdate) {
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
fn send_registration_update(_update: RegistrationUpdate) {}

#[cfg(target_arch = "wasm32")]
fn registration_ws_url() -> String {
    let Some(window) = web_sys::window() else {
        return "ws://127.0.0.1:3000/ws/registrations".to_string();
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
        return format!("http://127.0.0.1:3000{path}");
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
        Some(port) if is_dev_frontend_port(&port) => {
            let hostname = location
                .hostname()
                .unwrap_or_else(|_| "127.0.0.1".to_string());
            format!("{hostname}:3000")
        }
        _ => location
            .host()
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
    }
}

#[cfg(target_arch = "wasm32")]
fn is_dev_frontend_port(port: &str) -> bool {
    !port.is_empty() && port != "3000"
}
