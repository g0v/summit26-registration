use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate, Worker};

pub fn attendee_by_ticket(
    attendees: ReadSignal<Vec<Attendee>>,
    ticket_id: &str,
) -> Option<Attendee> {
    attendees
        .get()
        .into_iter()
        .find(|attendee| attendee.ticket_id == ticket_id)
}

pub fn sample_attendees() -> Vec<Attendee> {
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

pub fn worker_by_ticket(workers: ReadSignal<Vec<Worker>>, ticket_id: &str) -> Option<Worker> {
    workers
        .get()
        .into_iter()
        .find(|worker| worker.ticket_id == ticket_id)
}

pub fn sample_workers() -> Vec<Worker> {
    [
        ("沒有人", "CONF-0420", "自然組", "小農", false),
        ("Heisenberg W.", "CONF-1337", "化學組", "組長", false),
        ("JoJo", "CONF-5489", "社會組", "組頭", false),
    ]
    .into_iter()
    .map(|(nickname, ticket_id, team, role, registered)| Worker {
        nickname: nickname.to_string(),
        ticket_id: ticket_id.to_string(),
        team: team.to_string(),
        role: role.to_string(),
        registered,
    })
    .collect()
}

pub fn apply_attendee_registration_update(
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

pub fn apply_worker_registration_update(
    set_workers: WriteSignal<Vec<Worker>>,
    update: &RegistrationUpdate,
) {
    set_workers.update(|workers| {
        if let Some(worker) = workers
            .iter_mut()
            .find(|worker| worker.ticket_id == update.ticket_id)
        {
            worker.registered = update.registered;
        }
    });
}
