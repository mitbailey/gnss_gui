mod app;
pub use app::GenCamGUI;

#[cfg(target_arch = "wasm32")]
mod web;
