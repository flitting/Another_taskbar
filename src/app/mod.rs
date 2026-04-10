mod cli;
mod gui;

pub use cli::run_cli;
pub use gui::run_gui;

/// Parse the command-line arguments to determine which mode to run.
/// Returns "gui" by default, or "cli" if --cli/-c is specified.
/// Exits the program if --help/-h is specified or invalid arguments are provided.
pub fn parse_mode(args: &[String]) -> &'static str {
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "--cli" | "-c" => "cli",
            "--gui" | "-g" => "gui",
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: '{}'\n", arg);
                print_help();
                std::process::exit(1);
            }
        }
    } else {
        "gui"
    }
}

/// Print help message with usage information.
fn print_help() {
    println!("CLI Taskbar - Task Management Application");
    println!();
    println!("USAGE:");
    println!("    cli_taskbar [MODE]");
    println!();
    println!("MODES:");
    println!("    --gui, -g       Run in GUI mode (default)");
    println!("    --cli, -c       Run in CLI mode");
    println!("    --help, -h      Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cli_taskbar                 # Run in GUI mode");
    println!("    cli_taskbar --cli           # Run in CLI mode");
    println!("    cli_taskbar --gui           # Explicitly run in GUI mode");
    println!("    cli_taskbar --help          # Show this help message");
}
