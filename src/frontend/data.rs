use leptos::prelude::*;

use crate::models::{Attendee, RegistrationUpdate};

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

pub fn apply_registration_update(
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
