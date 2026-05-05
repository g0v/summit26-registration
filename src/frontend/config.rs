pub const BACKEND_PUBLIC_URL: &str = env!("SUMMIT26_BACKEND_PUBLIC_URL");

pub fn api_url(path: &str) -> String {
    if BACKEND_PUBLIC_URL.is_empty() {
        path.to_string()
    } else {
        format!("{BACKEND_PUBLIC_URL}{path}")
    }
}

pub fn websocket_url(path: &str, location: Option<&web_sys::Location>) -> String {
    if !BACKEND_PUBLIC_URL.is_empty() {
        return format!(
            "{}{}",
            websocket_base_url(BACKEND_PUBLIC_URL, location),
            path
        );
    }

    format!("{}{}", current_websocket_origin(location), path)
}

fn websocket_base_url(url: &str, location: Option<&web_sys::Location>) -> String {
    if let Some(rest) = url.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = url.strip_prefix("http://") {
        format!("ws://{rest}")
    } else if url.starts_with('/') {
        format!("{}{}", current_websocket_origin(location), url)
    } else {
        url.to_string()
    }
}

fn current_websocket_origin(location: Option<&web_sys::Location>) -> String {
    let Some(location) = location else {
        return "ws://127.0.0.1".to_string();
    };
    let protocol = if location.protocol().ok().as_deref() == Some("https:") {
        "wss"
    } else {
        "ws"
    };
    let host = location.host().unwrap_or_else(|_| "127.0.0.1".to_string());

    format!("{protocol}://{host}")
}
