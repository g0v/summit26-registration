use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate, Worker};

use super::{
    data::{
        apply_attendee_registration_update, apply_worker_registration_update, attendee_by_ticket,
        sample_attendees, sample_workers, worker_by_ticket,
    },
    sync::{
        send_attendee_registration_update, send_worker_registration_update, start_backend_sync,
    },
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum ActiveTab {
    Attendees,
    Workers,
}

#[component]
pub fn App() -> impl IntoView {
    let (attendees, set_attendees) = signal(sample_attendees());
    let (workers, set_workers) = signal(sample_workers());
    let (active_tab, set_active_tab) = signal(ActiveTab::Attendees);
    let (sync_status, set_sync_status) = signal("Sample Data".to_string());
    let (authenticated, set_authenticated) = signal(false);

    start_backend_sync(
        set_attendees,
        set_workers,
        set_sync_status,
        set_authenticated,
    );

    view! {
        <main class="app-shell">
            <header class="topbar">
                <div>
                    <p class="eyebrow">"Conference Registration"</p>
                    <h1>"Check-in Dashboard"</h1>
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

            <section class="tabbar" aria-label="Registration table selection">
                <button
                    type="button"
                    class=move || if active_tab.get() == ActiveTab::Attendees {
                        "tab-button is-active"
                    } else {
                        "tab-button"
                    }
                    on:click=move |_| set_active_tab.set(ActiveTab::Attendees)
                >
                    "Attendees"
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == ActiveTab::Workers {
                        "tab-button is-active"
                    } else {
                        "tab-button"
                    }
                    on:click=move |_| set_active_tab.set(ActiveTab::Workers)
                >
                    "Workers"
                </button>
            </section>

            {move || match active_tab.get() {
                ActiveTab::Attendees => view! {
                    <AttendeesTable
                        attendees=attendees
                        set_attendees=set_attendees
                        can_update=authenticated
                    />
                }.into_any(),
                ActiveTab::Workers => view! {
                    <WorkersTable
                        workers=workers
                        set_workers=set_workers
                        can_update=authenticated
                    />
                }.into_any(),
            }}
        </main>
    }
}

#[component]
fn AttendeesTable(
    attendees: ReadSignal<Vec<Attendee>>,
    set_attendees: WriteSignal<Vec<Attendee>>,
    can_update: ReadSignal<bool>,
) -> impl IntoView {
    view! {
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
                                    can_update=can_update
                                    on_toggle=Callback::new(move |update: RegistrationUpdate| {
                                        apply_attendee_registration_update(set_attendees, &update);
                                        send_attendee_registration_update(update);
                                    })
                                />
                            }
                        }
                    />
                </tbody>
            </table>
        </section>
    }
}

#[component]
fn WorkersTable(
    workers: ReadSignal<Vec<Worker>>,
    set_workers: WriteSignal<Vec<Worker>>,
    can_update: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <section class="table-wrap" aria-label="Worker registration spreadsheet">
            <table>
                <thead>
                    <tr>
                        <th scope="col">"Nickname"</th>
                        <th scope="col">"Ticket ID"</th>
                        <th scope="col">"Team"</th>
                        <th scope="col">"Role"</th>
                        <th scope="col" class="check-cell">"Registered"</th>
                        <th scope="col">"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || {
                            workers
                                .get()
                                .into_iter()
                                .map(|worker| worker.ticket_id)
                                .collect::<Vec<_>>()
                        }
                        key=|ticket_id| ticket_id.clone()
                        children=move |ticket_id| {
                            view! {
                                <WorkerRow
                                    ticket_id= ticket_id
                                    workers=workers
                                    can_update=can_update
                                    on_toggle=Callback::new(move |update: RegistrationUpdate| {
                                        apply_worker_registration_update(set_workers, &update);
                                        send_worker_registration_update(update);
                                    })
                                />
                            }
                        }
                    />
                </tbody>
            </table>
        </section>
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
    let label_ticket_id = ticket_id.clone();

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
            <RegistrationCells
                ticket_id=change_ticket_id
                label_ticket_id=aria_ticket_id
                registered=move || attendee_by_ticket(attendees, &checked_ticket_id)
                    .is_some_and(|attendee| attendee.registered)
                label=move || attendee_by_ticket(attendees, &label_ticket_id)
                    .map(|attendee| attendee.name)
                    .unwrap_or_else(|| label_ticket_id.clone())
                status_registered=move || attendee_by_ticket(attendees, &status_class_ticket_id)
                    .is_some_and(|attendee| attendee.registered)
                status_text_registered=move || attendee_by_ticket(attendees, &status_text_ticket_id)
                    .is_some_and(|attendee| attendee.registered)
                can_update=can_update
                on_toggle=on_toggle
            />
        </tr>
    }
}

#[component]
fn WorkerRow(
    ticket_id: String,
    workers: ReadSignal<Vec<Worker>>,
    can_update: ReadSignal<bool>,
    #[prop(into)] on_toggle: Callback<RegistrationUpdate>,
) -> impl IntoView {
    let row_ticket_id = ticket_id.clone();
    let nickname_cell_ticket_id = ticket_id.clone();
    let ticket_cell_ticket_id = ticket_id.clone();
    let team_cell_ticket_id = ticket_id.clone();
    let role_cell_ticket_id = ticket_id.clone();
    let aria_ticket_id = ticket_id.clone();
    let checked_ticket_id = ticket_id.clone();
    let change_ticket_id = ticket_id.clone();
    let status_class_ticket_id = ticket_id.clone();
    let status_text_ticket_id = ticket_id.clone();
    let label_ticket_id = ticket_id.clone();

    view! {
        <tr class:is-registered=move || {
            worker_by_ticket(workers, &row_ticket_id)
                .is_some_and(|worker| worker.registered)
        }>
            <td class="attendee-name">
                {move || {
                    worker_by_ticket(workers, &nickname_cell_ticket_id)
                        .map(|worker| worker.nickname)
                        .unwrap_or_default()
                }}
            </td>
            <td class="ticket-id">{ticket_cell_ticket_id}</td>
            <td>
                <span class="ticket-type">
                    {move || {
                        worker_by_ticket(workers, &team_cell_ticket_id)
                            .map(|worker| worker.team)
                            .unwrap_or_default()
                    }}
                </span>
            </td>
            <td>
                {move || {
                    worker_by_ticket(workers, &role_cell_ticket_id)
                        .map(|worker| worker.role)
                        .unwrap_or_default()
                }}
            </td>
            <RegistrationCells
                ticket_id=change_ticket_id
                label_ticket_id=aria_ticket_id
                registered=move || worker_by_ticket(workers, &checked_ticket_id)
                    .is_some_and(|worker| worker.registered)
                label=move || worker_by_ticket(workers, &label_ticket_id)
                    .map(|worker| worker.nickname)
                    .unwrap_or_else(|| label_ticket_id.clone())
                status_registered=move || worker_by_ticket(workers, &status_class_ticket_id)
                    .is_some_and(|worker| worker.registered)
                status_text_registered=move || worker_by_ticket(workers, &status_text_ticket_id)
                    .is_some_and(|worker| worker.registered)
                can_update=can_update
                on_toggle=on_toggle
            />
        </tr>
    }
}

#[component]
fn RegistrationCells(
    ticket_id: String,
    label_ticket_id: String,
    registered: impl Fn() -> bool + Send + Sync + 'static,
    label: impl Fn() -> String + Send + Sync + 'static,
    status_registered: impl Fn() -> bool + Send + Sync + 'static,
    status_text_registered: impl Fn() -> bool + Send + Sync + 'static,
    can_update: ReadSignal<bool>,
    #[prop(into)] on_toggle: Callback<RegistrationUpdate>,
) -> impl IntoView {
    let input_ticket_id = ticket_id.clone();

    view! {
        <td class="check-cell">
            <label
                class=move || if can_update.get() {
                    "check-control"
                } else {
                    "check-control is-disabled"
                }
                aria-label=move || format!("Register {}", label())
                data-ticket-id=label_ticket_id
            >
                <input
                    type="checkbox"
                    prop:checked=registered
                    prop:disabled=move || !can_update.get()
                    on:change=move |event| {
                        if !can_update.get() {
                            return;
                        }

                        on_toggle.run(RegistrationUpdate {
                            ticket_id: input_ticket_id.clone(),
                            registered: event_target_checked(&event),
                        });
                    }
                />
            </label>
        </td>
        <td>
            <span class=move || if status_registered() {
                "registered-label"
            } else {
                "pending-label"
            }>
                {move || if status_text_registered() {
                    "Registered"
                } else {
                    "Pending"
                }}
            </span>
        </td>
    }
}
