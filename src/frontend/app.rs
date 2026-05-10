use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate};

use super::{
    data::{apply_registration_update, attendee_by_ticket, sample_attendees},
    sync::{send_registration_update, start_backend_sync},
};

#[component]
pub fn App() -> impl IntoView {
    let (attendees, set_attendees) = signal(sample_attendees());
    let (sync_status, set_sync_status) = signal("Sample data".to_string());
    let (authenticated, set_authenticated) = signal(false);

    start_backend_sync(set_attendees, set_sync_status, set_authenticated);

    view! {
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
                                        can_update=authenticated
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
    can_update: ReadSignal<bool>,
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
    let disabled_label_can_update = can_update;
    let disabled_input_can_update = can_update;
    let change_can_update = can_update;

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
                    class=move || if disabled_label_can_update.get() {
                        "check-control"
                    } else {
                        "check-control is-disabled"
                    }
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
                        prop:disabled=move || !disabled_input_can_update.get()
                        on:change=move |event| {
                            if !change_can_update.get() {
                                return;
                            }

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
