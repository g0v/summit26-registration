use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attendee {
    pub name: String,
    pub ticket_id: String,
    pub ticket_type: String,
    pub registered: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrationUpdate {
    pub ticket_id: String,
    pub registered: bool,
}
