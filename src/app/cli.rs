use crate::files::{load_taskbar, save_taskbar, DEFAULT_TASKBAR_PATH};
use crate::input_parse::{parse_input, CliAction};
use crate::tasks::TaskManager;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::io::{self, Write};
use std::path::PathBuf;

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
                "delete all".to_string(),
                "list".to_string(),
                "show".to_string(),
                "stats".to_string(),
                "save".to_string(),
                "load".to_string(),
                "setting".to_string(),
                "filter".to_string(),
                "search".to_string(),
                "undo".to_string(),
                "exit".to_string(),
                "quit".to_string(),
            ],
        }
    }

    fn filter_keywords() -> Vec<&'static str> {
        vec![
            "--list",
            "--ls",
            "--clear",
            "--clear_all",
            "--importance",
            "--urgency",
            "--state",
            "--pinned",
            "high",
            "low",
            "neither",
            "todo",
            "inprogress",
            "blocked",
            "completed",
            "archived",
            "none",
            "true",
            "false",
            "clear",
        ]
    }

    fn setting_keywords() -> Vec<&'static str> {
        vec!["theme", "font", "show_details_aside", "true", "false"]
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
        if let Some(prefix) = trimmed.strip_prefix("help ") {
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| {
                    cmd.to_ascii_lowercase()
                        .starts_with(&prefix.to_ascii_lowercase())
                })
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            let start_pos = line.len() - prefix.len();
            return Ok((start_pos, matches));
        }
        if let Some(prefix) = trimmed.strip_prefix("filter ") {
            let fragment = prefix.split_whitespace().last().unwrap_or(prefix);
            let matches: Vec<Pair> = Self::filter_keywords()
                .into_iter()
                .filter(|keyword| {
                    keyword
                        .to_ascii_lowercase()
                        .starts_with(&fragment.to_ascii_lowercase())
                })
                .map(|keyword| Pair {
                    display: keyword.to_string(),
                    replacement: keyword.to_string(),
                })
                .collect();
            let start_pos = line.len() - fragment.len();
            return Ok((start_pos, matches));
        }
        if let Some(prefix) = trimmed.strip_prefix("setting ") {
            let fragment = prefix.split_whitespace().last().unwrap_or(prefix);
            let matches: Vec<Pair> = Self::setting_keywords()
                .into_iter()
                .filter(|keyword| {
                    keyword
                        .to_ascii_lowercase()
                        .starts_with(&fragment.to_ascii_lowercase())
                })
                .map(|keyword| Pair {
                    display: keyword.to_string(),
                    replacement: keyword.to_string(),
                })
                .collect();
            let start_pos = line.len() - fragment.len();
            return Ok((start_pos, matches));
        }
        if let Some(prefix) = trimmed.strip_prefix("delete ") {
            let target = format!("delete {}", prefix);
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| {
                    cmd.to_ascii_lowercase()
                        .starts_with(&target.to_ascii_lowercase())
                })
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            let start_pos = line.len() - trimmed.len();
            Ok((start_pos, matches))
        } else {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() <= 1 {
                let prefix = parts.first().unwrap_or(&"");
                let matches: Vec<Pair> = self
                    .commands
                    .iter()
                    .filter(|cmd| {
                        cmd.to_ascii_lowercase()
                            .starts_with(&prefix.to_ascii_lowercase())
                    })
                    .map(|cmd| Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    })
                    .collect();

                let start_pos = line.len() - prefix.len();
                Ok((start_pos, matches))
            } else {
                Ok((line.len(), vec![]))
            }
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
pub fn run_cli() {
    let mut path = PathBuf::from(DEFAULT_TASKBAR_PATH);

    let mut manager = match load_taskbar(path.as_path()) {
        Ok(m) => {
            println!("Loaded taskbar from {}", path.display());
            m
        }
        Err(_) => {
            if !ask_startup_choice() {
                println!("Exiting...");
                return;
            }
            println!("Starting with a new taskbar.");
            TaskManager::new()
        }
    };

    let completer = CommandCompleter::new();
    let mut editor = Editor::with_config(Default::default()).unwrap();
    editor.set_helper(Some(completer));

    loop {
        let readline = editor.readline("> ");

        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                editor.add_history_entry(&line).ok();

                match parse_input(line.trim(), &mut manager, &mut path) {
                    CliAction::Continue => {}
                    CliAction::Exit => break,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\nEOF received, saving and exiting...");
                if let Err(e) = save_taskbar(path.as_path(), &manager) {
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
