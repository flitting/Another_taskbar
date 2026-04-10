use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::files::{
    backup_taskbar, count_pinned_tasks, display_all_tasks, get_task_stats, load_taskbar,
    save_taskbar, taskbar_file_exists,
};
use crate::gui::settings::{
    apply_saved_theme, import_theme_file, load_gui_settings, load_theme_palette, save_gui_settings,
};
use crate::tasks::*;

use super::prompts::{
    read_line, read_optional_importance, read_optional_string, read_optional_urgency, read_pinned,
    read_required_string, read_task_state,
};
use super::utils::{
    auto_save, eq_ci, format_selected_filters, parse_importance, parse_importance_filter,
    parse_next_string, parse_next_u32, parse_optional_file_arg, parse_pinned_filter,
    parse_state_filter, parse_tags_value, parse_task_state, parse_urgency, parse_urgency_filter,
    print_filter_state, show_search_results,
};
use super::CliAction;

pub(crate) fn run_command_group(
    tokens: &[String],
    manager: &mut TaskManager,
    current_path: &mut PathBuf,
) -> CliAction {
    let command = tokens.first().map(String::as_str).unwrap_or("");

    if eq_ci(command, "help") {
        print_help(tokens.get(1..).unwrap_or(&[]));
    } else if eq_ci(command, "add") {
        add_task(&tokens[1..], manager, current_path)
    } else if eq_ci(command, "list") {
        display_all_tasks(manager)
    } else if eq_ci(command, "show") {
        let id_str = tokens.get(1).map(String::as_str).unwrap_or("");
        show_task(id_str, manager);
    } else if eq_ci(command, "update") {
        let id_str = tokens.get(1).map(String::as_str).unwrap_or("");
        update_task(id_str, &tokens[2..], manager, current_path);
    } else if eq_ci(command, "delete") {
        handle_delete(&tokens[1..], manager, current_path);
    } else if eq_ci(command, "stats") {
        println!("{}", get_task_stats(manager));
        println!("  Pinned: {}", count_pinned_tasks(manager));
    } else if eq_ci(command, "save") {
        save_command(&tokens[1..], manager, current_path)
    } else if eq_ci(command, "load") {
        load_command(&tokens[1..], manager, current_path)
    } else if eq_ci(command, "setting") {
        setting_command(&tokens[1..])
    } else if eq_ci(command, "filter") {
        filter_command(&tokens[1..], manager)
    } else if eq_ci(command, "search") {
        search_command(&tokens[1..], manager)
    } else if eq_ci(command, "undo") {
        match manager.undo_last_change() {
            Ok(_) => {
                println!("Last change undone.");
                auto_save(manager, current_path);
            }
            Err(error) => println!("{error}"),
        }
    } else if eq_ci(command, "exit") || eq_ci(command, "quit") {
        match save_taskbar(current_path.as_path(), manager) {
            Ok(_) => println!("Saved to {}", current_path.display()),
            Err(error) => println!("Save failed: {error}"),
        }
        println!("Goodbye");
        return CliAction::Exit;
    } else {
        println!("Unknown command: {command}");
    }

    CliAction::Continue
}

fn print_help(args: &[String]) {
    if args.is_empty() {
        println!(
            "Commands:
  add [options]                 - Add a task
  update <id> [options]         - Update a task
  delete <id>                   - Delete one task
  delete all [--yes]            - Delete all tasks
  list                          - List tasks
  show <id>                     - Show detailed information about a task
  stats                         - Show task statistics
  save [--file FILEPATH]        - Save tasks
  load [--file FILEPATH]        - Load tasks
  setting NAME VALUE            - Update a GUI setting
  filter ...                    - Manage tag filters
  search \"STRING\"               - Search tasks by name or description
  undo                          - Undo the last change
  exit | quit                   - Save and exit
  help                          - Show this message
  help COMMAND_NAME             - Show exact help for one command

Examples:
  add --name \"Write docs\" --pinned save --file work.json
  load --file work.json list stats
  update 3 --state completed --tags done,docs save"
        );
        return;
    }

    let topic = args.join(" ");
    let topic_normalized = topic.to_lowercase();
    match topic_normalized.as_str() {
        "add" => println!(
            "help add
  add [--parent ID] [--name NAME] [--description TEXT] [--state STATE]
      [--urgency low|high] [--importance low|high] [--tags a,b,c] [--pinned]

  If no automation flags are provided, the CLI falls back to interactive prompts."
        ),
        "update" => println!(
            "help update
  update <id> [--name NAME] [--description TEXT] [--state STATE]
         [--urgency low|high|none] [--importance low|high|none]
         [--tags a,b,c] [--pinned true|false]

  If no field options are provided, the CLI falls back to the interactive update menu."
        ),
        "delete" => println!(
            "help delete
  delete <id>         Delete one task
  delete all [--yes]  Delete all tasks after backup. Use --yes to skip confirmation."
        ),
        "delete all" => println!(
            "help delete all
  delete all [--yes]

  Creates a backup of the current file before clearing every task."
        ),
        "save" => println!(
            "help save
  save [--file FILEPATH]

  Saves to the current active file by default.
  If --file is provided, it saves there and switches the active file to that path."
        ),
        "load" => println!(
            "help load
  load [--file FILEPATH]

  Loads from the current active file by default.
  If --file is provided, it loads that file and switches the active file to that path."
        ),
        "setting" => println!(
            "help setting
  setting theme THEME_PATH
  setting show_details_aside true|false

  Updates persisted GUI settings. Boolean settings use true or false."
        ),
        "filter" => println!(
            "help filter
  filter --list | --ls
  filter --importance high|low|neither
  filter --urgency high|low|neither
  filter --state Todo|InProgress|Blocked|Completed|Archived|None
  filter --pinned true|false|clear
  filter TAG1 TAG2 ...
  filter --clear --importance
  filter --clear --urgency
  filter --clear --state
  filter --clear TAG1 TAG2 ...
  filter --clear_all

  Shows and edits the active filter set."
        ),
        "search" => println!(
            "help search
  search \"STRING\"
  search --clear

  Searches task names and descriptions. Search is separate from filters and does not affect undo."
        ),
        "undo" => println!("help undo\n  undo\n\n  Reverts the last undoable task change."),
        "list" => println!("help list\n  list\n\n  Displays all tasks."),
        "show" => {
            println!("help show\n  show <id>\n\n  Displays detailed information for one task.")
        }
        "stats" => println!("help stats\n  stats\n\n  Displays aggregate task statistics."),
        "help" => println!(
            "help help\n  help [COMMAND_NAME]\n\n  Shows general or command-specific help."
        ),
        "exit" | "quit" => {
            println!("help exit\n  exit | quit\n\n  Saves to the active file and exits.")
        }
        other => println!("No exact help found for '{other}'."),
    }
}

fn add_task(args: &[String], manager: &mut TaskManager, current_path: &Path) {
    if !args.is_empty() && args.iter().any(|arg| arg.starts_with("--")) {
        add_task_automated(args, manager, current_path);
        return;
    }

    let father_id = if !args.is_empty() {
        args[0].parse::<u32>().unwrap_or(0)
    } else {
        0
    };

    let name = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        read_required_string("Task name")
    };

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
    let tags = parse_tags_value(&tags_str);
    let pinned = read_pinned();

    let draft = TaskDraft {
        name,
        description,
        state,
        urgency,
        importance,
        tags,
        pinned,
        due_date: None,
        completed_at: None,
    };

    match manager.create_task_from_draft(father_id, draft) {
        Ok(id) => {
            println!("Task added successfully with id {id}!");
            auto_save(manager, current_path);
        }
        Err(error) => println!("Error adding task: {error}"),
    }
}

fn add_task_automated(args: &[String], manager: &mut TaskManager, current_path: &Path) {
    let mut parent_id = 0;
    let mut name: Option<String> = None;
    let mut description = String::new();
    let mut state = TaskState::Todo;
    let mut urgency = None;
    let mut importance = None;
    let mut tags = Vec::new();
    let mut pinned = false;

    let mut index = 0;
    while index < args.len() {
        match args[index].to_ascii_lowercase().as_str() {
            "--parent" => {
                parent_id = parse_next_u32(args, &mut index, "--parent").unwrap_or(0);
            }
            "--name" => {
                name = parse_next_string(args, &mut index, "--name");
            }
            "--description" => {
                description =
                    parse_next_string(args, &mut index, "--description").unwrap_or_default();
            }
            "--state" => {
                let value = parse_next_string(args, &mut index, "--state").unwrap_or_default();
                state = match parse_task_state(&value) {
                    Some(state) => state,
                    None => {
                        println!("Invalid state: {value}");
                        return;
                    }
                };
            }
            "--urgency" => {
                let value = parse_next_string(args, &mut index, "--urgency").unwrap_or_default();
                urgency = match value.to_ascii_lowercase().as_str() {
                    "none" => None,
                    _ => match parse_urgency(&value) {
                        Some(value) => Some(value),
                        None => {
                            println!("Invalid urgency: {value}");
                            return;
                        }
                    },
                };
            }
            "--importance" => {
                let value = parse_next_string(args, &mut index, "--importance").unwrap_or_default();
                importance = match value.to_ascii_lowercase().as_str() {
                    "none" => None,
                    _ => match parse_importance(&value) {
                        Some(value) => Some(value),
                        None => {
                            println!("Invalid importance: {value}");
                            return;
                        }
                    },
                };
            }
            "--tags" => {
                let value = parse_next_string(args, &mut index, "--tags").unwrap_or_default();
                tags = parse_tags_value(&value);
            }
            "--pinned" => {
                pinned = true;
            }
            other => {
                println!("Unknown add option: {other}");
                return;
            }
        }
        index += 1;
    }

    let Some(name) = name else {
        println!("add automation requires --name.");
        return;
    };

    let draft = TaskDraft {
        name,
        description,
        state,
        urgency,
        importance,
        tags,
        pinned,
        due_date: None,
        completed_at: None,
    };

    match manager.create_task_from_draft(parent_id, draft) {
        Ok(id) => {
            println!("Task added successfully with id {id}!");
            auto_save(manager, current_path);
        }
        Err(error) => println!("Error adding task: {error}"),
    }
}

fn show_task(id_str: &str, manager: &mut TaskManager) {
    let id: u32 = if id_str.trim().is_empty() {
        let value = read_required_string("Task ID");
        value.parse().unwrap_or(0)
    } else {
        id_str.parse().unwrap_or(0)
    };
    if id == 0 {
        println!("Task id 0 is the internal root task and cannot be shown directly.");
        return;
    }
    if let Some(task) = manager.root.search_by_id(id) {
        task.display_detail();
    } else {
        println!("Task not found");
    }
}

fn update_task(id_str: &str, args: &[String], manager: &mut TaskManager, current_path: &Path) {
    let id: u32 = id_str.parse().unwrap_or(0);
    if id == 0 {
        println!("Task id 0 is the internal root task and cannot be updated.");
        return;
    }
    if args.is_empty() {
        update_task_interactive(id, manager, current_path);
    } else {
        update_task_automated(id, args, manager, current_path);
    }
}

fn update_task_interactive(id: u32, manager: &mut TaskManager, current_path: &Path) {
    loop {
        if manager.root.search_by_id_ref(id).is_none() {
            println!("Task not found");
            return;
        }

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

        if eq_ci(&choice, "q") {
            break;
        }

        let Some(current_task) = manager.root.search_by_id_ref(id) else {
            println!("Task not found");
            return;
        };
        let mut draft = TaskDraft::from(current_task);

        match choice.as_str() {
            "1" => draft.name = read_required_string("New name"),
            "2" => draft.description = read_optional_string("New description"),
            "3" => draft.state = read_task_state(),
            "4" => draft.urgency = read_optional_urgency(),
            "5" => draft.importance = read_optional_importance(),
            "6" => {
                let tags_str = read_optional_string("New tags (comma-separated)");
                draft.tags = parse_tags_value(&tags_str);
            }
            "7" => {
                draft.pinned = loop {
                    print!("Pinned (y/n): ");
                    io::stdout().flush().unwrap();
                    let input = read_line();
                    match input.to_lowercase().as_str() {
                        "y" | "yes" => break true,
                        "n" | "no" => break false,
                        _ => println!("Please enter y or n"),
                    }
                };
            }
            _ => {
                println!("Invalid choice");
                continue;
            }
        }

        if let Err(error) = manager.update_task_from_draft(id, draft) {
            println!("Error updating task: {error}");
            continue;
        }
        auto_save(manager, current_path);

        if let Some(task) = manager.root.search_by_id_ref(id) {
            println!("\nUpdated! Current state:");
            task.display_detail();
        }
    }
}

fn update_task_automated(id: u32, args: &[String], manager: &mut TaskManager, current_path: &Path) {
    let Some(current_task) = manager.root.search_by_id_ref(id) else {
        println!("Task not found");
        return;
    };

    let mut draft = TaskDraft::from(current_task);
    let mut changed = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            _ if eq_ci(&args[index], "--name") => {
                draft.name = parse_next_string(args, &mut index, "--name").unwrap_or_default();
                changed = true;
            }
            _ if eq_ci(&args[index], "--description") => {
                draft.description =
                    parse_next_string(args, &mut index, "--description").unwrap_or_default();
                changed = true;
            }
            _ if eq_ci(&args[index], "--state") => {
                let value = parse_next_string(args, &mut index, "--state").unwrap_or_default();
                let Some(state) = parse_task_state(&value) else {
                    println!("Invalid state: {value}");
                    return;
                };
                draft.state = state;
                changed = true;
            }
            _ if eq_ci(&args[index], "--urgency") => {
                let value = parse_next_string(args, &mut index, "--urgency").unwrap_or_default();
                draft.urgency = if eq_ci(&value, "none") {
                    None
                } else {
                    let Some(value) = parse_urgency(&value) else {
                        println!("Invalid urgency: {value}");
                        return;
                    };
                    Some(value)
                };
                changed = true;
            }
            _ if eq_ci(&args[index], "--importance") => {
                let value = parse_next_string(args, &mut index, "--importance").unwrap_or_default();
                draft.importance = if eq_ci(&value, "none") {
                    None
                } else {
                    let Some(value) = parse_importance(&value) else {
                        println!("Invalid importance: {value}");
                        return;
                    };
                    Some(value)
                };
                changed = true;
            }
            _ if eq_ci(&args[index], "--tags") => {
                let value = parse_next_string(args, &mut index, "--tags").unwrap_or_default();
                draft.tags = parse_tags_value(&value);
                changed = true;
            }
            _ if eq_ci(&args[index], "--pinned") => {
                let value = parse_next_string(args, &mut index, "--pinned").unwrap_or_default();
                draft.pinned = match value.to_ascii_lowercase().as_str() {
                    "true" | "yes" | "y" | "1" => true,
                    "false" | "no" | "n" | "0" => false,
                    _ => {
                        println!("Invalid pinned value: {value}");
                        return;
                    }
                };
                changed = true;
            }
            other => {
                println!("Unknown update option: {other}");
                return;
            }
        }
        index += 1;
    }

    if !changed {
        println!("No update options supplied.");
        return;
    }

    match manager.update_task_from_draft(id, draft) {
        Ok(()) => {
            println!("Task updated successfully.");
            auto_save(manager, current_path);
        }
        Err(error) => println!("Error updating task: {error}"),
    }
}

fn handle_delete(args: &[String], manager: &mut TaskManager, current_path: &Path) {
    if args.first().is_some_and(|arg| eq_ci(arg, "all")) {
        let skip_confirm = args.iter().any(|arg| eq_ci(arg, "--yes"));
        delete_all_tasks(manager, current_path, skip_confirm);
        return;
    }

    let id: u32 = args
        .first()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0);
    if id == 0 {
        println!("Task id 0 is the internal root task and cannot be deleted.");
        return;
    }

    if manager.delete_task(id).is_ok() {
        println!("Task deleted successfully!");
        auto_save(manager, current_path);
    } else {
        println!("Task not found");
    }
}

fn delete_all_tasks(manager: &mut TaskManager, current_path: &Path, skip_confirm: bool) {
    if !skip_confirm {
        print!("Are you sure you want to delete all tasks? This will create a backup. (y/n): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        if io::stdin().read_line(&mut confirm).is_err() {
            println!("Delete all cancelled.");
            return;
        }
        match confirm.trim().to_lowercase().as_str() {
            "y" | "yes" => {}
            _ => {
                println!("Delete all cancelled.");
                return;
            }
        }
    }

    if taskbar_file_exists(current_path) {
        let backup_path = format!("{}.backup", current_path.display());
        match backup_taskbar(current_path, Path::new(&backup_path)) {
            Ok(_) => println!("Backup created at {backup_path}"),
            Err(error) => {
                println!("Could not create backup, aborting delete all: {error}");
                return;
            }
        }
    }

    manager.clear_tasks();
    println!("All tasks have been deleted!");
    auto_save(manager, current_path);
}

fn save_command(args: &[String], manager: &TaskManager, current_path: &mut PathBuf) {
    let path = match parse_optional_file_arg(args) {
        Ok(path) => path.unwrap_or_else(|| current_path.clone()),
        Err(()) => return,
    };

    match save_taskbar(path.as_path(), manager) {
        Ok(_) => {
            println!("Saved to {}", path.display());
            *current_path = path;
        }
        Err(error) => println!("Save failed: {error}"),
    }
}

fn load_command(args: &[String], manager: &mut TaskManager, current_path: &mut PathBuf) {
    let path = match parse_optional_file_arg(args) {
        Ok(path) => path.unwrap_or_else(|| current_path.clone()),
        Err(()) => return,
    };

    match load_taskbar(path.as_path()) {
        Ok(loaded) => {
            *manager = loaded;
            *current_path = path.clone();
            println!("Loaded from {}", path.display());
        }
        Err(error) => println!("Load failed: {error}"),
    }
}

fn setting_command(args: &[String]) {
    let Some(setting_name) = args.first() else {
        println!("Usage: setting NAME VALUE");
        return;
    };
    let Some(setting_value) = args.get(1) else {
        println!("Missing value for setting '{}'.", setting_name);
        return;
    };

    if eq_ci(setting_name, "theme") {
        let path = Path::new(setting_value);
        let imported_name = match import_theme_file(path) {
            Ok(name) => name,
            Err(error) => {
                println!("{error}");
                return;
            }
        };

        if let Err(error) = load_theme_palette(&imported_name) {
            println!("{error}");
            return;
        }

        let mut settings = load_gui_settings();
        settings.selected_theme = imported_name.clone();

        match save_gui_settings(&settings) {
            Ok(()) => {
                let _ = apply_saved_theme();
                println!("Theme set to '{}'.", imported_name);
            }
            Err(error) => println!("{error}"),
        }
        return;
    }

    if eq_ci(setting_name, "show_details_aside") {
        let value = match setting_value.to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => {
                println!(
                    "Invalid value for show_details_aside: {}. Use true or false.",
                    setting_value
                );
                return;
            }
        };

        let mut settings = load_gui_settings();
        settings.show_details_aside = value;
        match save_gui_settings(&settings) {
            Ok(()) => println!("show_details_aside set to {}.", value),
            Err(error) => println!("{error}"),
        }
        return;
    }

    println!(
        "Unknown setting '{}'. Supported settings: theme, show_details_aside.",
        setting_name
    );
}

fn filter_command(args: &[String], manager: &mut TaskManager) {
    if args.is_empty()
        || args
            .iter()
            .any(|arg| eq_ci(arg, "--list") || eq_ci(arg, "--ls"))
    {
        print_filter_state(manager);
        return;
    }

    if args.first().is_some_and(|arg| eq_ci(arg, "--clear_all")) {
        manager.clear_all_filters();
        println!("Cleared all selected filters.");
        display_all_tasks(manager);
        return;
    }

    if args.first().is_some_and(|arg| eq_ci(arg, "--clear")) {
        if args.len() == 1 {
            println!("Nothing to clear.");
            return;
        }

        let mut index = 1;
        while index < args.len() {
            match args[index].to_ascii_lowercase().as_str() {
                "--importance" => manager.set_active_importance_filter(ImportanceFilter::Any),
                "--urgency" => manager.set_active_urgency_filter(UrgencyFilter::Any),
                "--state" => manager.set_active_state_filter(StateFilter::Any),
                "--pinned" => manager.set_active_pinned_filter(PinnedFilter::Any),
                tag => manager.clear_active_filter_tag(tag),
            }
            index += 1;
        }
        println!(
            "Updated selected filters: {}",
            format_selected_filters(manager)
        );
        display_all_tasks(manager);
        return;
    }

    let mut index = 0;
    while index < args.len() {
        match args[index].to_ascii_lowercase().as_str() {
            "--importance" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    println!("Missing value for --importance");
                    return;
                };
                let Some(filter) = parse_importance_filter(value) else {
                    println!("Invalid importance filter: {value}");
                    return;
                };
                manager.set_active_importance_filter(filter);
            }
            "--urgency" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    println!("Missing value for --urgency");
                    return;
                };
                let Some(filter) = parse_urgency_filter(value) else {
                    println!("Invalid urgency filter: {value}");
                    return;
                };
                manager.set_active_urgency_filter(filter);
            }
            "--state" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    println!("Missing value for --state");
                    return;
                };
                let Some(filter) = parse_state_filter(value) else {
                    println!("Invalid state filter: {value}");
                    return;
                };
                manager.set_active_state_filter(filter);
            }
            "--pinned" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    println!("Missing value for --pinned");
                    return;
                };
                let Some(filter) = parse_pinned_filter(value) else {
                    println!("Invalid pinned filter: {value}");
                    return;
                };
                manager.set_active_pinned_filter(filter);
            }
            tag => manager.toggle_active_filter_tag(tag),
        }
        index += 1;
    }
    println!(
        "Updated selected filters: {}",
        format_selected_filters(manager)
    );
    display_all_tasks(manager);
}

fn search_command(args: &[String], manager: &mut TaskManager) {
    if args.is_empty() {
        if manager.has_active_search() {
            println!("Active search: {}", manager.active_search_query);
        } else {
            println!("No active search.");
        }
        show_search_results(manager);
        return;
    }

    if args.first().is_some_and(|arg| eq_ci(arg, "--clear")) {
        manager.clear_active_search_query();
        println!("Cleared search.");
        show_search_results(manager);
        return;
    }

    manager.set_active_search_query(args.join(" "));
    println!("Active search: {}", manager.active_search_query);
    show_search_results(manager);
}
