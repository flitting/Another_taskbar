use crate::tasks::*;
use chrono::Utc;
use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_required_string(prompt: &str) -> String {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        let input = read_line();
        if !input.is_empty() {
            return input;
        }
        println!("This field is required. Please enter a value.");
    }
}

fn read_optional_string(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    read_line()
}

fn parse_task_state(input: &str) -> Option<TaskState> {
    match input.trim().to_lowercase().as_str() {
        "todo" | "0" => Some(TaskState::Todo),
        "inprogress" | "in_progress" | "1" => Some(TaskState::InProgress),
        "blocked" | "2" => Some(TaskState::Blocked),
        "completed" | "3" => Some(TaskState::Completed),
        "archived" | "4" => Some(TaskState::Archived),
        _ => None,
    }
}

fn parse_urgency(input: &str) -> Option<TaskUrgency> {
    match input.trim().to_lowercase().as_str() {
        "low" | "0" => Some(TaskUrgency::Low),
        "high" | "1" => Some(TaskUrgency::High),
        _ => None,
    }
}

fn parse_importance(input: &str) -> Option<TaskImportance> {
    match input.trim().to_lowercase().as_str() {
        "low" | "0" => Some(TaskImportance::Low),
        "high" | "1" => Some(TaskImportance::High),
        _ => None,
    }
}

fn read_task_state() -> TaskState {
    loop {
        println!("State (0=Todo, 1=InProgress, 2=Blocked, 3=Completed, 4=Archived):");
        let input = read_line();
        if let Some(state) = parse_task_state(&input) {
            return state;
        }
        println!("Invalid state. Please try again.");
    }
}

fn read_optional_urgency() -> Option<TaskUrgency> {
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

fn read_optional_importance() -> Option<TaskImportance> {
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

fn read_pinned() -> bool {
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

fn resort_all_subtasks(task: &mut Task) {
    task.sort_subtasks();
    for subtask in &mut task.subtasks {
        resort_all_subtasks(subtask);
    }
}

pub fn parse_input(input: &str, manager: &mut TaskManager) {
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();

    match command {
        "add" => {
            add_task(parts.collect::<Vec<&str>>(), manager);
        }
        "list" => {
            manager.root.display_single();
        }
        "show" => {
            let id_str = parts.next().unwrap_or("");
            show_task(id_str, manager);
        }
        "update" => {
            let id_str = parts.next().unwrap_or("");
            update_task(id_str, manager);
        }
        "delete" => {
            let id_str = parts.next().unwrap_or("");
            let id: u32 = id_str.parse().unwrap_or(0);
            if let Some(_) = manager.root.remove_subtask(id) {
                println!("Task deleted successfully!");
            } else {
                println!("Task not found");
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn add_task(args: Vec<&str>, manager: &mut TaskManager) {
    // Try to parse optional father_id from args
    let father_id = if !args.is_empty() {
        args[0].parse::<u32>().unwrap_or(0)
    } else {
        0
    };

    // Ask for task name if not provided via args
    let name = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        read_required_string("Task name")
    };

    // If we got a name from args but no father_id was provided, ask for it
    let father_id = if args.is_empty() {
        let father_id_input = read_optional_string("Father task ID (optional, default=0)");
        if father_id_input.is_empty() {
            0
        } else {
            father_id_input.parse().unwrap_or(0)
        }
    } else {
        father_id
    };

    let description = read_optional_string("Description (optional)");
    let state = read_task_state();
    let urgency = read_optional_urgency();
    let importance = read_optional_importance();

    let tags_str = read_optional_string("Tags (comma-separated, optional)");
    let tags: Vec<String> = if tags_str.is_empty() {
        Vec::new()
    } else {
        tags_str.split(',').map(|s| s.trim().to_string()).collect()
    };

    let pinned = read_pinned();

    let times = TaskTimes {
        created_at: Utc::now(),
        updated_at: Utc::now(),
        due_date: None,
        completed_at: None,
    };

    // Let TaskManager assign the ID by passing 0 so next_id() is used and
    // IDs continue from the highest loaded/used value.
    let task = Task::new(
        0,
        name,
        description,
        state,
        urgency,
        importance,
        tags,
        pinned,
        times,
        0,
    );

    match manager.add_task(father_id, task) {
        Ok(_) => {
            println!("Task added successfully!");
        }
        Err(e) => println!("Error adding task: {}", e),
    }
}

fn show_task(id_str: &str, manager: &mut TaskManager) {
    let id: u32 = id_str.parse().unwrap_or(0);
    if let Some(task) = manager.root.search_by_id(id) {
        task.display_detail();
    } else {
        println!("Task not found");
    }
}

fn update_task(id_str: &str, manager: &mut TaskManager) {
    let id: u32 = id_str.parse().unwrap_or(0);

    loop {
        // Check if task exists
        if manager.root.search_by_id(id).is_none() {
            println!("Task not found");
            return;
        }

        // Display update menu
        println!("\nWhat you want to change?");
        println!("1. name");
        println!("2. description");
        println!("3. state");
        println!("4. urgency");
        println!("5. importance");
        println!("6. tags");
        println!("7. pinned");
        println!("q. quit");
        print!("> ");
        io::stdout().flush().unwrap();

        let choice = read_line();

        if choice == "q" {
            break;
        }

        let old_pinned = manager.root.search_by_id(id).map(|t| t.pinned);
        let old_due_date = manager.root.search_by_id(id).map(|t| t.times.due_date);

        let mut update = TaskUpdate::default();
        let mut needs_resort = false;

        match choice.as_str() {
            "1" => {
                update.name = Some(read_required_string("New name"));
            }
            "2" => {
                update.description = Some(read_optional_string("New description"));
            }
            "3" => {
                update.state = Some(read_task_state());
            }
            "4" => {
                update.urgency = read_optional_urgency();
            }
            "5" => {
                update.importance = read_optional_importance();
            }
            "6" => {
                let tags_str = read_optional_string("New tags (comma-separated)");
                update.tags = Some(if tags_str.is_empty() {
                    Vec::new()
                } else {
                    tags_str.split(',').map(|s| s.trim().to_string()).collect()
                });
            }
            "7" => {
                update.pinned = Some(loop {
                    print!("Pinned (y/n): ");
                    io::stdout().flush().unwrap();
                    let input = read_line();
                    match input.to_lowercase().as_str() {
                        "y" | "yes" => break true,
                        "n" | "no" => break false,
                        _ => println!("Please enter y or n"),
                    }
                });
                needs_resort = true;
            }
            _ => {
                println!("Invalid choice");
                continue;
            }
        }

        // Apply the update
        if let Some(task) = manager.root.search_by_id(id) {
            task.change_field(update);

            // Check if due_date changed
            if old_due_date != Some(task.times.due_date) {
                needs_resort = true;
            }

            // Check if pinned status changed
            if old_pinned.is_some() && old_pinned != Some(task.pinned) {
                needs_resort = true;
            }

            println!("\nUpdated! Current state:");
            task.display_detail();

            // Resort all subtasks in the tree if needed
            if needs_resort {
                resort_all_subtasks(&mut manager.root);
            }
        }
    }
}
