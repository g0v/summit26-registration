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
        (
            "CONF-1027",
            Some("Speaker"),
            Some("Vegetarian"),
            Some(true),
            true,
        ),
        ("CONF-1184", Some("VIP"), None, Some(false), false),
        ("CONF-1266", Some("General"), Some("Vegan"), None, false),
        (
            "CONF-1315",
            Some("Workshop"),
            Some("None"),
            Some(true),
            true,
        ),
        (
            "CONF-1442",
            Some("General"),
            Some("Halal"),
            Some(false),
            false,
        ),
        ("CONF-1503", Some("Sponsor"), None, Some(true), false),
        (
            "CONF-1638",
            Some("VIP"),
            Some("Vegetarian"),
            Some(true),
            true,
        ),
        ("CONF-1790", None, None, None, false),
    ]
    .into_iter()
    .map(
        |(ticket_id, ticket_type, meal_preference, reception, registered)| Attendee {
            ticket_id: ticket_id.to_string(),
            ticket_type: ticket_type.map(str::to_string),
            meal_preference: meal_preference.map(str::to_string),
            reception,
            registered,
        },
    )
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
