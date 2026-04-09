use std::env;

mod app;
mod cli_display;
mod files;
mod gui;
mod input_parse;
mod symbols;
mod tasks;

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
