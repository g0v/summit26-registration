use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=APP_CONFIG");
    println!("cargo:rerun-if-env-changed=APP__FRONTEND__BACKEND_PUBLIC_URL");
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=config.example.toml");

    let config_path = env::var("APP_CONFIG").unwrap_or_else(|_| {
        if Path::new("config.toml").exists() {
            "config.toml".to_string()
        } else {
            "config.example.toml".to_string()
        }
    });

    let mut backend_public_url = read_backend_public_url(&config_path);

    if let Ok(value) = env::var("APP__FRONTEND__BACKEND_PUBLIC_URL") {
        backend_public_url = value;
    }

    println!("cargo:rustc-env=SUMMIT26_BACKEND_PUBLIC_URL={backend_public_url}");
}

fn read_backend_public_url(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    let parsed = content.parse::<toml::Table>().unwrap_or_default();

    let configured_backend_url = parsed
        .get("frontend")
        .and_then(toml::Value::as_table)
        .and_then(|frontend| frontend.get("backend_public_url"))
        .and_then(toml::Value::as_str)
        .unwrap_or("")
        .trim_end_matches('/')
        .to_string();

    if !configured_backend_url.is_empty() {
        return configured_backend_url;
    }

    if is_development(&parsed) {
        return local_backend_url(&parsed);
    }

    String::new()
}

fn is_development(config: &toml::Table) -> bool {
    config
        .get("auth")
        .and_then(toml::Value::as_table)
        .and_then(|auth| auth.get("development"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
}

fn local_backend_url(config: &toml::Table) -> String {
    let server = config.get("server").and_then(toml::Value::as_table);
    let bind_host = server
        .and_then(|server| server.get("bind_host").or_else(|| server.get("host")))
        .and_then(toml::Value::as_str)
        .unwrap_or("127.0.0.1");
    let port = server
        .and_then(|server| server.get("port"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(3000);
    let host = match bind_host {
        "" | "0.0.0.0" | "::" => "127.0.0.1",
        host => host,
    };

    format!("http://{host}:{port}")
}
