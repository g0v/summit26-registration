use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::models::RegistrationUpdate;

use super::{config::AuthSettings, rest_client::VerifierApiClient};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub registrations: broadcast::Sender<RegistrationUpdate>,
    pub verifier_api: VerifierApiClient,
    pub auth: AuthSettings,
}

impl AppState {
    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}
