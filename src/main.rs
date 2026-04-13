#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;

use another_taskbar::app;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command-line arguments (skip program name at index 0)
    let mode = app::parse_mode(&args);

    match mode {
        "cli" => app::run_cli(),
        "gui" => app::run_gui(),
        _ => unreachable!(),
    }
}
