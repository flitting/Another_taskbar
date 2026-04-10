mod commands;
mod prompts;
mod utils;

use std::path::PathBuf;

use crate::tasks::TaskManager;

pub(crate) use commands::run_command_group;
pub(crate) use utils::eq_ci;

pub enum CliAction {
    Continue,
    Exit,
}

const ROOT_COMMANDS: &[&str] = &[
    "help",
    "add",
    "update",
    "delete",
    "list",
    "show",
    "stats",
    "save",
    "load",
    "setting",
    "filter",
    "search",
    "undo",
    "wipe-data",
    "exit",
    "quit",
];

pub fn parse_input(
    input: &str,
    manager: &mut TaskManager,
    current_path: &mut PathBuf,
) -> CliAction {
    let Some(tokens) = shlex::split(input) else {
        println!("Could not parse input. Check your quotes.");
        return CliAction::Continue;
    };

    if tokens.is_empty() {
        return CliAction::Continue;
    }

    let groups = split_command_groups(&tokens);

    for group in groups {
        if matches!(
            run_command_group(&group, manager, current_path),
            CliAction::Exit
        ) {
            return CliAction::Exit;
        }
    }

    CliAction::Continue
}

fn split_command_groups(tokens: &[String]) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let command = tokens[start].as_str();
        let mut end = start + 1;

        if eq_ci(command, "help") {
            if end < tokens.len() {
                end += 1;
                if tokens
                    .get(start + 1)
                    .map(String::as_str)
                    .is_some_and(|token| eq_ci(token, "delete"))
                    && tokens
                        .get(start + 2)
                        .map(String::as_str)
                        .is_some_and(|token| eq_ci(token, "all"))
                {
                    end += 1;
                }
            }
        } else {
            while end < tokens.len() && !is_root_command(&tokens[end]) {
                end += 1;
            }
        }

        groups.push(tokens[start..end].to_vec());
        start = end;
    }

    groups
}

fn is_root_command(token: &str) -> bool {
    ROOT_COMMANDS.iter().any(|command| eq_ci(token, command))
}
