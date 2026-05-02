use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};

use super::state::AppState;

pub async fn handle_registration_socket(socket: WebSocket, state: Arc<AppState>) {
    let mut updates = state.registrations.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(update) = updates.recv().await {
            let Ok(message) = serde_json::to_string(&update) else {
                continue;
            };

            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = receiver.next().await {
        if matches!(message, Message::Close(_)) {
            break;
        }
    }

    send_task.abort();
}
