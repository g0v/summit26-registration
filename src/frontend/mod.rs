mod app;
#[cfg(target_arch = "wasm32")]
mod config;
mod data;
mod sync;

pub use app::App;
