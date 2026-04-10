use std::io::{self, Write};

use crate::tasks::{TaskImportance, TaskState, TaskUrgency};

use super::utils::{parse_importance, parse_task_state, parse_urgency};

pub(crate) fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub(crate) fn read_required_string(prompt: &str) -> String {
    loop {
        print!("{prompt}: ");
        io::stdout().flush().unwrap();
        let input = read_line();
        if !input.is_empty() {
            return input;
        }
        println!("This field is required. Please enter a value.");
    }
}

pub(crate) fn read_optional_string(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    read_line()
}

pub(crate) fn read_task_state() -> TaskState {
    loop {
        println!("State (0=Todo, 1=InProgress, 2=Blocked, 3=Completed, 4=Archived):");
        let input = read_line();
        if let Some(state) = parse_task_state(&input) {
            return state;
        }
        println!("Invalid state. Please try again.");
    }
}

pub(crate) fn read_optional_urgency() -> Option<TaskUrgency> {
    loop {
        println!("Urgency (0=Low, 1=High, or blank to skip):");
        let input = read_line();
        if input.is_empty() {
            return None;
        }
        if let Some(urgency) = parse_urgency(&input) {
            return Some(urgency);
        }
        println!("Invalid urgency. Please try again or leave blank.");
    }
}

pub(crate) fn read_optional_importance() -> Option<TaskImportance> {
    loop {
        println!("Importance (0=Low, 1=High, or blank to skip):");
        let input = read_line();
        if input.is_empty() {
            return None;
        }
        if let Some(importance) = parse_importance(&input) {
            return Some(importance);
        }
        println!("Invalid importance. Please try again or leave blank.");
    }
}

pub(crate) fn read_pinned() -> bool {
    loop {
        print!("Pinned (y/n, default=n): ");
        io::stdout().flush().unwrap();
        let input = read_line();
        match input.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" | "" => return false,
            _ => println!("Please enter y or n"),
        }
    }
}
