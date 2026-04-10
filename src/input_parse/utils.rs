use std::path::{Path, PathBuf};

use crate::files::{display_all_tasks, save_taskbar};
use crate::tasks::{
    ImportanceFilter, PinnedFilter, StateFilter, TaskImportance, TaskManager, TaskState,
    TaskUrgency, UrgencyFilter,
};

pub(crate) fn eq_ci(value: &str, expected: &str) -> bool {
    value.eq_ignore_ascii_case(expected)
}

pub(crate) fn parse_task_state(input: &str) -> Option<TaskState> {
    match input.trim().to_lowercase().as_str() {
        "todo" | "0" => Some(TaskState::Todo),
        "inprogress" | "in_progress" | "1" => Some(TaskState::InProgress),
        "blocked" | "2" => Some(TaskState::Blocked),
        "completed" | "3" => Some(TaskState::Completed),
        "archived" | "4" => Some(TaskState::Archived),
        _ => None,
    }
}

pub(crate) fn parse_urgency(input: &str) -> Option<TaskUrgency> {
    match input.trim().to_lowercase().as_str() {
        "low" | "0" => Some(TaskUrgency::Low),
        "high" | "1" => Some(TaskUrgency::High),
        _ => None,
    }
}

pub(crate) fn parse_importance(input: &str) -> Option<TaskImportance> {
    match input.trim().to_lowercase().as_str() {
        "low" | "0" => Some(TaskImportance::Low),
        "high" | "1" => Some(TaskImportance::High),
        _ => None,
    }
}

pub(crate) fn parse_importance_filter(input: &str) -> Option<ImportanceFilter> {
    match input.trim().to_lowercase().as_str() {
        "high" => Some(ImportanceFilter::High),
        "low" => Some(ImportanceFilter::Low),
        "neither" => Some(ImportanceFilter::Neither),
        "any" => Some(ImportanceFilter::Any),
        _ => None,
    }
}

pub(crate) fn parse_urgency_filter(input: &str) -> Option<UrgencyFilter> {
    match input.trim().to_lowercase().as_str() {
        "high" => Some(UrgencyFilter::High),
        "low" => Some(UrgencyFilter::Low),
        "neither" => Some(UrgencyFilter::Neither),
        "any" => Some(UrgencyFilter::Any),
        _ => None,
    }
}

pub(crate) fn parse_pinned_filter(input: &str) -> Option<PinnedFilter> {
    match input.trim().to_lowercase().as_str() {
        "true" | "pinned" => Some(PinnedFilter::Pinned),
        "false" | "unpinned" => Some(PinnedFilter::Unpinned),
        "clear" | "any" => Some(PinnedFilter::Any),
        _ => None,
    }
}

pub(crate) fn parse_state_filter(input: &str) -> Option<StateFilter> {
    match input.trim().to_lowercase().as_str() {
        "todo" => Some(StateFilter::Todo),
        "inprogress" | "in_progress" => Some(StateFilter::InProgress),
        "blocked" => Some(StateFilter::Blocked),
        "completed" => Some(StateFilter::Completed),
        "archived" => Some(StateFilter::Archived),
        "none" => Some(StateFilter::None),
        "any" | "clear" => Some(StateFilter::Any),
        _ => None,
    }
}

pub(crate) fn parse_optional_file_arg(args: &[String]) -> Result<Option<PathBuf>, ()> {
    if let Some(first) = args.first() {
        let mut index = 0;
        match first.to_ascii_lowercase().as_str() {
            "--file" => {
                return Ok(parse_next_string(args, &mut index, "--file").map(PathBuf::from));
            }
            other => {
                println!("Unknown option: {other}");
                return Err(());
            }
        };
    }
    Ok(None)
}

pub(crate) fn parse_tags_value(value: &str) -> Vec<String> {
    if value.trim().is_empty() {
        Vec::new()
    } else {
        value
            .split(',')
            .map(|segment| segment.trim().to_string())
            .filter(|segment| !segment.is_empty())
            .collect()
    }
}

pub(crate) fn auto_save(manager: &TaskManager, current_path: &Path) {
    match save_taskbar(current_path, manager) {
        Ok(()) => println!("Saved to {}", current_path.display()),
        Err(error) => println!("Auto-save failed: {error}"),
    }
}

pub(crate) fn print_filter_state(manager: &TaskManager) {
    let available = if manager.available_tags.is_empty() {
        "none".to_string()
    } else {
        manager.available_tags.join(", ")
    };
    println!("Available tags: {available}");
    println!("Selected filters: {}", format_selected_filters(manager));
}

pub(crate) fn format_selected_filters(manager: &TaskManager) -> String {
    let mut parts = Vec::new();

    if !manager.active_filter_tags.is_empty() {
        parts.push(format!("tags={}", manager.active_filter_tags.join(", ")));
    }

    if !matches!(manager.active_importance_filter, ImportanceFilter::Any) {
        parts.push(format!(
            "importance={}",
            format_importance_filter(&manager.active_importance_filter)
        ));
    }

    if !matches!(manager.active_urgency_filter, UrgencyFilter::Any) {
        parts.push(format!(
            "urgency={}",
            format_urgency_filter(&manager.active_urgency_filter)
        ));
    }

    if !matches!(manager.active_state_filter, StateFilter::Any) {
        parts.push(format!(
            "state={}",
            format_state_filter(&manager.active_state_filter)
        ));
    }

    if !matches!(manager.active_pinned_filter, PinnedFilter::Any) {
        parts.push(format!(
            "pinned={}",
            format_pinned_filter(&manager.active_pinned_filter)
        ));
    }

    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(" | ")
    }
}

fn format_importance_filter(filter: &ImportanceFilter) -> &'static str {
    match filter {
        ImportanceFilter::Any => "any",
        ImportanceFilter::High => "high",
        ImportanceFilter::Low => "low",
        ImportanceFilter::Neither => "neither",
    }
}

fn format_urgency_filter(filter: &UrgencyFilter) -> &'static str {
    match filter {
        UrgencyFilter::Any => "any",
        UrgencyFilter::High => "high",
        UrgencyFilter::Low => "low",
        UrgencyFilter::Neither => "neither",
    }
}

fn format_pinned_filter(filter: &PinnedFilter) -> &'static str {
    match filter {
        PinnedFilter::Any => "any",
        PinnedFilter::Pinned => "true",
        PinnedFilter::Unpinned => "false",
    }
}

fn format_state_filter(filter: &StateFilter) -> &'static str {
    match filter {
        StateFilter::Any => "any",
        StateFilter::Todo => "Todo",
        StateFilter::InProgress => "InProgress",
        StateFilter::Blocked => "Blocked",
        StateFilter::Completed => "Completed",
        StateFilter::Archived => "Archived",
        StateFilter::None => "None",
    }
}

pub(crate) fn parse_next_string(args: &[String], index: &mut usize, flag: &str) -> Option<String> {
    *index += 1;
    let value = args.get(*index).cloned();
    if value.is_none() {
        println!("Missing value for {flag}");
    }
    value
}

pub(crate) fn parse_next_u32(args: &[String], index: &mut usize, flag: &str) -> Option<u32> {
    let value = parse_next_string(args, index, flag)?;
    match value.parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid numeric value for {flag}: {value}");
            None
        }
    }
}

pub(crate) fn show_search_results(manager: &TaskManager) {
    display_all_tasks(manager);
}
