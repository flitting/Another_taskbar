mod cli;
mod gui;

pub use cli::run_cli;
pub use gui::run_gui;

/// Entry point for the application (backward compatibility).
/// Delegates to CLI mode by default.
pub fn run() {
    run_cli();
}

/// Parse the command-line arguments to determine which mode to run.
/// Returns "cli" by default, or "gui" if --gui/-g is specified.
/// Exits the program if --help/-h is specified or invalid arguments are provided.
pub fn parse_mode(args: &[String]) -> &'static str {
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--cli" | "-c" => return "cli",
            "--gui" | "-g" => return "gui",
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
    }

    "cli"
}

/// Print help message with usage information.
fn print_help() {
    println!("CLI Taskbar - Task Management Application");
    println!();
    println!("USAGE:");
    println!("    cli_taskbar [MODE]");
    println!();
    println!("MODES:");
    println!("    --cli, -c       Run in CLI mode (default)");
    println!("    --gui, -g       Run in GUI mode (coming soon)");
    println!("    --help, -h      Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cli_taskbar                 # Run in CLI mode (default)");
    println!("    cli_taskbar --cli           # Explicitly run in CLI mode");
    println!("    cli_taskbar --gui           # Run in GUI mode (not yet implemented)");
    println!("    cli_taskbar --help          # Show this help message");
}
