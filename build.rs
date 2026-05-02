use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=APP_CONFIG");
    println!("cargo:rerun-if-env-changed=APP__SERVER__HOST");
    println!("cargo:rerun-if-env-changed=APP__SERVER__PORT");
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=config.example.toml");

    let config_path = env::var("APP_CONFIG").unwrap_or_else(|_| {
        if Path::new("config.toml").exists() {
            "config.toml".to_string()
        } else {
            "config.example.toml".to_string()
        }
    });

    let (mut host, mut port) = read_server_config(&config_path);

    if let Ok(value) = env::var("APP__SERVER__HOST") {
        host = value;
    }

    if let Ok(value) = env::var("APP__SERVER__PORT") {
        port = value;
    }

    println!("cargo:rustc-env=SUMMIT26_SERVER_HOST={host}");
    println!("cargo:rustc-env=SUMMIT26_SERVER_PORT={port}");
}

fn read_server_config(path: &str) -> (String, String) {
    let content = fs::read_to_string(path)
        .unwrap_or_default();
    let parsed = content.parse::<toml::Table>().unwrap_or_default();
    let server = parsed.get("server").and_then(toml::Value::as_table);

    let host = server
        .and_then(|server| server.get("host"))
        .and_then(toml::Value::as_str)
        .unwrap_or("127.0.0.1")
        .to_string();
    let port = server
        .and_then(|server| server.get("port"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(3000)
        .to_string();

    (host, port)
}
