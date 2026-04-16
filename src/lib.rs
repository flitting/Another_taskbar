pub mod app;
pub mod app_paths;
pub mod bootstrap;
pub mod cli_display;
pub mod files;
pub mod gui;
pub mod input_parse;
pub mod locale;
pub mod symbols;
pub mod tasks;

// Tauri Android mobile entry point
#[cfg_attr(target_os = "android", tauri::mobile_entry_point)]
pub async fn run() {
    app::run_gui();
}
