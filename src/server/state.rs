use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::models::RegistrationUpdate;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub registrations: broadcast::Sender<RegistrationUpdate>,
}

impl AppState {
    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}
