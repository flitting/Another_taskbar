use crate::files::{
    backup_taskbar, clear_all_tasks, count_pinned_tasks, display_all_tasks, get_task_stats,
    load_taskbar, save_taskbar,
};
use crate::gui;
use crate::input_parse::parse_input;
use crate::tasks::TaskManager;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::io::{self, Write};

/// Custom completer for command auto-completion
struct CommandCompleter {
    commands: Vec<String>,
}

impl CommandCompleter {
    fn new() -> Self {
        CommandCompleter {
            commands: vec![
                "help".to_string(),
                "add".to_string(),
                "update".to_string(),
                "delete".to_string(),
                "list".to_string(),
                "stats".to_string(),
                "save".to_string(),
                "load".to_string(),
                "clear".to_string(),
                "exit".to_string(),
                "quit".to_string(),
            ],
        }
    }
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let trimmed = line.trim_start();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        // If we're completing the first word (command), suggest matching commands
        if parts.len() <= 1 {
            let prefix = parts.get(0).unwrap_or(&"");
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();

            let start_pos = line.len() - prefix.len();
            Ok((start_pos, matches))
        } else {
            // For now, don't complete arguments
            Ok((line.len(), vec![]))
        }
    }
}

impl Hinter for CommandCompleter {
    type Hint = String;
}

impl Highlighter for CommandCompleter {}

impl Validator for CommandCompleter {}

impl Helper for CommandCompleter {}

/// Ask the user at startup whether they want to create a new taskbar or load an existing one.
/// Returns true if the user wants to proceed, false if they want to quit.
fn ask_startup_choice() -> bool {
    loop {
        print!("No taskbar found. Do you want to create a new taskbar? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => println!("Please enter 'y' or 'n'."),
            },
            Err(_) => {
                println!("Error reading input. Please try again.");
            }
        }
    }
}

/// Run the interactive taskbar application loop in CLI mode.
/// Loads taskbar from `taskbar.json` if present, otherwise asks the user if they want to create one.
/// Supports commands:
///  - help         : show this help
///  - add ...      : interactive add (see `add` implementation)
///  - update <id>  : interactive update
///  - delete <id>  : delete a task
///  - list         : display tasks
///  - stats        : show task statistics (counts by state and pinned)
///  - save         : save to disk
///  - load         : reload from disk (overwrites current in-memory state)
///  - clear        : clear all tasks (with confirmation and backup)
///  - exit|quit    : save and exit
pub fn run_cli() {
    let path = "taskbar.json";

    // Try to load existing taskbar
    let mut manager = match load_taskbar(path) {
        Ok(m) => {
            println!("Loaded taskbar from {}", path);
            m
        }
        Err(_) => {
            // Taskbar file doesn't exist or couldn't be loaded
            if !ask_startup_choice() {
                println!("Exiting...");
                return;
            }
            println!("Starting with a new taskbar.");
            TaskManager::new()
        }
    };

    // Initialize rustyline editor for auto-completion
    let completer = CommandCompleter::new();
    let mut editor = Editor::with_config(Default::default()).unwrap();
    let _completer = editor.set_helper(Some(completer));

    loop {
        // Read input with auto-completion support
        let readline = editor.readline("> ");

        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                // Add to history
                editor.add_history_entry(&line).ok();

                let input_line = line.trim();

                match input_line {
                    "help" => {
                        println!(
                            "Commands:
  add [father_id] [name]  - Add a task (interactive prompts follow)
  update <id>             - Update a task interactively
  delete <id>             - Delete a task
  list                    - List tasks
  show <id>               - Show detailed information about a task
  stats                   - Show task statistics
  save                    - Save to taskbar.json
  load                    - Load from taskbar.json (replace current state)
  clear                   - Clear all tasks (creates backup first)
  exit | quit             - Save and exit
  help                    - Show this message"
                        );
                    }

                    "save" => match save_taskbar(path, &manager) {
                        Ok(_) => println!("Saved to {}", path),
                        Err(e) => println!("Save failed: {}", e),
                    },

                    "load" => match load_taskbar(path) {
                        Ok(m) => {
                            manager = m;
                            println!("Loaded from {}", path);
                        }
                        Err(e) => println!("Load failed: {}", e),
                    },

                    "list" => {
                        // display all tasks without showing root
                        display_all_tasks(&manager);
                    }

                    "stats" => {
                        // Display task statistics
                        println!("{}", get_task_stats(&manager));
                        let pinned = count_pinned_tasks(&manager);
                        println!("  Pinned: {}", pinned);
                    }

                    "clear" => {
                        // Ask for confirmation before clearing
                        print!(
                            "Are you sure you want to clear all tasks? This will create a backup. (y/n): "
                        );
                        io::stdout().flush().unwrap();
                        let mut confirm = String::new();
                        if io::stdin().read_line(&mut confirm).is_ok() {
                            match confirm.trim().to_lowercase().as_str() {
                                "y" | "yes" => {
                                    // Create a backup before clearing
                                    let backup_path = format!("{}.backup", path);
                                    match backup_taskbar(path, &backup_path) {
                                        Ok(_) => {
                                            println!("Backup created at {}", backup_path);
                                            manager = clear_all_tasks();
                                            println!("All tasks have been cleared!");
                                        }
                                        Err(e) => {
                                            println!(
                                                "Could not create backup, aborting clear: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                                _ => println!("Clear cancelled."),
                            }
                        }
                    }

                    "exit" | "quit" => {
                        match save_taskbar(path, &manager) {
                            Ok(_) => println!("Saved to {}", path),
                            Err(e) => println!("Save failed: {}", e),
                        }
                        println!("Goodbye");
                        break;
                    }

                    other => {
                        // Delegate parsing/handling to parse_input which can be interactive
                        // and will mutate the manager as needed.
                        parse_input(other, &mut manager);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\nEOF received, saving and exiting...");
                if let Err(e) = save_taskbar(path, &manager) {
                    println!("Failed to save on exit: {}", e);
                }
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        }
    }
}

/// Run the taskbar application in GUI mode.
/// Loads the taskbar and launches an iced window with the task manager.
pub fn run_gui() {
    match gui::run_gui_app() {
        Ok(_) => println!("GUI closed successfully"),
        Err(e) => eprintln!("GUI error: {}", e),
    }
}

/// Entry point for the application (backward compatibility).
/// Delegates to CLI mode by default.
pub fn run() {
    run_cli();
}

/// Parse the command-line arguments to determine which mode to run.
/// Returns "cli" by default, or "gui" if --gui/-g is specified.
/// Exits the program if --help/-h is specified or invalid arguments are provided.
pub fn parse_mode(args: &[String]) -> &'static str {
    // Skip the first argument (program name)
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

    // Default to CLI mode
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
