#[cfg(not(target_arch = "wasm32"))]
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attendee {
    pub name: String,
    pub ticket_id: String,
    pub ticket_type: String,
    pub registered: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Worker {
    pub nickname: String,
    pub ticket_id: String,
    pub team: String,
    pub role: String,
    pub registered: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrationUpdate {
    pub ticket_id: String,
    pub registered: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrationTable {
    Attendees,
    Workers,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrationEvent {
    pub table: RegistrationTable,
    pub ticket_id: String,
    pub registered: bool,
}

impl RegistrationEvent {
    pub fn new(table: RegistrationTable, update: RegistrationUpdate) -> Self {
        Self {
            table,
            ticket_id: update.ticket_id,
            registered: update.registered,
        }
    }

    pub fn update(&self) -> RegistrationUpdate {
        RegistrationUpdate {
            ticket_id: self.ticket_id.clone(),
            registered: self.registered,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpDeeplinkQuery {
    #[serde(rename = "vpUid")]
    pub vp_uid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpDeeplinkResponse {
    pub code: String,
    pub message: String,
    pub data: VpDeeplinkData,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpDeeplinkData {
    pub deep_link: String,
}

impl VpDeeplinkResponse {
    pub fn fixed(deep_link: impl Into<String>) -> Self {
        Self {
            code: "0".to_string(),
            message: "SUCCESS".to_string(),
            data: VpDeeplinkData {
                deep_link: deep_link.into(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodeDataRequest {
    #[serde(rename = "ref")]
    pub reference: String,
    pub transaction_id: String,
    pub is_callback: String,
}

impl QrCodeDataRequest {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn for_vp_uid(vp_uid: impl Into<String>) -> Self {
        let vp_uid = vp_uid.into();
        let reference_suffix = Alphanumeric.sample_string(&mut rand::rng(), 16);

        Self {
            reference: format!("{}", &vp_uid),
            transaction_id: format!("{}-transaction-{}", &vp_uid, &reference_suffix),
            is_callback: "Y".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodeDataResponse {
    pub transaction_id: String,
    pub qrcode_image: String,
    pub auth_uri: String,
}
