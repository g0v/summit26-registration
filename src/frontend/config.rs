pub const SERVER_HOST: &str = env!("SUMMIT26_SERVER_HOST");
pub const SERVER_PORT: &str = env!("SUMMIT26_SERVER_PORT");

pub fn configured_backend_host(location: Option<&web_sys::Location>) -> String {
    let host = if is_bind_all_host(SERVER_HOST) {
        location
            .and_then(|location| location.hostname().ok())
            .filter(|hostname| !hostname.is_empty())
            .unwrap_or_else(|| "127.0.0.1".to_string())
    } else {
        SERVER_HOST.to_string()
    };

    format!("{host}:{SERVER_PORT}")
}

pub fn is_configured_backend_port(port: &str) -> bool {
    port == SERVER_PORT
}

fn is_bind_all_host(host: &str) -> bool {
    matches!(host, "0.0.0.0" | "::")
}
