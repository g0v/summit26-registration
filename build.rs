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

    let mut backend_public_url = read_frontend_backend_public_url(&config_path);

    if let Ok(value) = env::var("APP__FRONTEND__BACKEND_PUBLIC_URL") {
        backend_public_url = value;
    }

    println!("cargo:rustc-env=SUMMIT26_BACKEND_PUBLIC_URL={backend_public_url}");
}

fn read_frontend_backend_public_url(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    let parsed = content.parse::<toml::Table>().unwrap_or_default();
    let frontend = parsed.get("frontend").and_then(toml::Value::as_table);

    frontend
        .and_then(|frontend| frontend.get("backend_public_url"))
        .and_then(toml::Value::as_str)
        .unwrap_or("")
        .trim_end_matches('/')
        .to_string()
}
