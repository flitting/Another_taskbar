// save the taskbar into json file and load taskbar from the json file.

use crate::app_paths;
use crate::tasks::*;
use serde_json::{from_str, to_string_pretty};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};

pub const DEFAULT_TASKBAR_FILE_NAME: &str = "tasks.json";

pub struct TaskbarDefaultPath;

impl TaskbarDefaultPath {
    pub fn resolve() -> Result<PathBuf, String> {
        app_paths::taskbar_path()
    }
}

/// Save a TaskManager to a JSON file at the specified path.
///
/// # Arguments
/// * `path` - The file path where the taskbar will be saved
/// * `manager` - A reference to the TaskManager to save
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` if serialization or file writing fails
pub fn save_taskbar<P: AsRef<Path>>(path: P, manager: &TaskManager) -> Result<(), String> {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create taskbar directory: {}", e))?;
    }

    let json =
        to_string_pretty(manager).map_err(|e| format!("Failed to serialize taskbar: {}", e))?;

    write(path, json).map_err(|e| format!("Failed to write taskbar file: {}", e))?;

    Ok(())
}

/// Validate and fix layer values in the task tree.
/// Ensures that each subtask has a layer equal to its parent's layer + 1.
/// If invalid layers are found, they are corrected.
///
/// # Arguments
/// * `task` - A mutable reference to the task to validate and fix
fn validate_and_fix_layers(task: &mut Task) {
    for subtask in &mut task.subtasks {
        // Set the correct layer for this subtask
        let expected_layer = task.layer + 1;
        if subtask.layer != expected_layer {
            subtask.layer = expected_layer;
        }
        // Recursively validate and fix nested subtasks
        validate_and_fix_layers(subtask);
    }
}

/// Load a TaskManager from a JSON file at the specified path.
///
/// # Arguments
/// * `path` - The file path to load the taskbar from
///
/// # Returns
/// * `Ok(TaskManager)` containing the loaded taskbar
/// * `Err(String)` if file reading or deserialization fails
pub fn load_taskbar<P: AsRef<Path>>(path: P) -> Result<TaskManager, String> {
    let content =
        read_to_string(path).map_err(|e| format!("Failed to read taskbar file: {}", e))?;

    let mut manager: TaskManager =
        from_str(&content).map_err(|e| format!("Failed to deserialize taskbar: {}", e))?;

    // Validate and fix layer values for all tasks in the tree
    validate_and_fix_layers(&mut manager.root);

    // Ensure the loaded TaskManager's internal unique-id counter
    // is at least the current maximum id found in the task tree.
    // This prevents next_id() from producing IDs that collide or restart.
    manager.ensure_uni_id();

    Ok(manager)
}

/// Display all tasks in the manager without showing the root task itself.
///
/// This function displays only the subtasks of the root task, providing
/// a clean view of the task hierarchy without the invisible root node.
///
/// # Arguments
/// * `manager` - A reference to the TaskManager to display
pub fn display_all_tasks(manager: &TaskManager) {
    let tasks = manager.filtered_tasks();

    if tasks.is_empty() {
        println!("No tasks. Use 'add' to create a new task.");
        return;
    }

    for subtask in &tasks {
        subtask.display_single();
    }
}

/// Check if a taskbar file exists at the specified path.
///
/// # Arguments
/// * `path` - The file path to check
///
/// # Returns
/// `true` if the file exists, `false` otherwise
pub fn taskbar_file_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// Create a backup of the taskbar file.
///
/// This function reads the source taskbar file and writes it to a backup location.
/// Useful for creating snapshots before major operations or migrations.
///
/// # Arguments
/// * `source_path` - The path of the taskbar file to back up
/// * `backup_path` - The path where the backup should be saved
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` if the backup operation fails
pub fn backup_taskbar<P: AsRef<Path>>(source_path: P, backup_path: P) -> Result<(), String> {
    let content = read_to_string(&source_path)
        .map_err(|e| format!("Failed to read taskbar file for backup: {}", e))?;

    write(backup_path, content).map_err(|e| format!("Failed to create backup: {}", e))?;

    Ok(())
}

/// Count the total number of tasks in the manager (excluding root).
///
/// # Arguments
/// * `manager` - A reference to the TaskManager
///
/// # Returns
/// The total count of all tasks including subtasks at all levels
pub fn count_all_tasks(manager: &TaskManager) -> usize {
    fn count_recursive(task: &Task) -> usize {
        let mut count = 1; // Count this task
        for subtask in &task.subtasks {
            count += count_recursive(subtask);
        }
        count
    }

    // Count all subtasks of root, excluding root itself (id=0)
    manager.root.subtasks.iter().map(count_recursive).sum()
}

/// Count tasks by their state.
///
/// # Arguments
/// * `manager` - A reference to the TaskManager
/// * `state_name` - The state to count ("todo", "inprogress", "blocked", "completed", "archived")
///
/// # Returns
/// The count of tasks in the specified state
pub fn count_tasks_by_state(manager: &TaskManager, state_name: &str) -> usize {
    fn count_recursive(task: &Task, state_name: &str) -> usize {
        let matches = match state_name.to_lowercase().as_str() {
            "todo" => matches!(task.state, TaskState::Todo),
            "inprogress" | "in_progress" => matches!(task.state, TaskState::InProgress),
            "blocked" => matches!(task.state, TaskState::Blocked),
            "completed" => matches!(task.state, TaskState::Completed),
            "archived" => matches!(task.state, TaskState::Archived),
            _ => false,
        };

        let mut count = if matches { 1 } else { 0 };
        for subtask in &task.subtasks {
            count += count_recursive(subtask, state_name);
        }
        count
    }

    // Count only subtasks of root, excluding root itself (id=0)
    manager
        .root
        .subtasks
        .iter()
        .map(|t| count_recursive(t, state_name))
        .sum()
}

/// Generate a statistics summary string.
///
/// # Arguments
/// * `manager` - A reference to the TaskManager
///
/// # Returns
/// A formatted string containing task statistics
pub fn get_task_stats(manager: &TaskManager) -> String {
    let total = count_all_tasks(manager);
    let todo = count_tasks_by_state(manager, "todo");
    let inprogress = count_tasks_by_state(manager, "inprogress");
    let blocked = count_tasks_by_state(manager, "blocked");
    let completed = count_tasks_by_state(manager, "completed");
    let archived = count_tasks_by_state(manager, "archived");

    format!(
        "Task Statistics:\n  Total: {}\n  Todo: {}\n  In Progress: {}\n  Blocked: {}\n  Completed: {}\n  Archived: {}",
        total, todo, inprogress, blocked, completed, archived
    )
}

/// Count pinned tasks in the manager.
///
/// # Arguments
/// * `manager` - A reference to the TaskManager
///
/// # Returns
/// The count of all pinned tasks
pub fn count_pinned_tasks(manager: &TaskManager) -> usize {
    fn count_recursive(task: &Task) -> usize {
        let mut count = if task.pinned { 1 } else { 0 };
        for subtask in &task.subtasks {
            count += count_recursive(subtask);
        }
        count
    }

    count_recursive(&manager.root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;

    fn create_test_task(id: u32, name: &str, state: TaskState, pinned: bool) -> Task {
        let times = TaskTimes {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            completed_at: None,
        };

        Task {
            id,
            name: name.to_string(),
            description: String::new(),
            state,
            urgency: None,
            importance: None,
            tags: Vec::new(),
            pinned,
            subtasks: Vec::new(),
            times,
            layer: 0,
            custom_order: 0,
            recurrence: None,
        }
    }

    #[test]
    fn test_count_all_tasks_empty() {
        let manager = TaskManager::new();
        assert_eq!(count_all_tasks(&manager), 0);
    }

    #[test]
    fn test_count_all_tasks_single() {
        let mut manager = TaskManager::new();
        let task = create_test_task(1, "Task 1", TaskState::Todo, false);
        manager.add_task(0, task).unwrap();

        assert_eq!(count_all_tasks(&manager), 1);
    }

    #[test]
    fn test_count_all_tasks_multiple() {
        let mut manager = TaskManager::new();

        for i in 1..=5 {
            let task = create_test_task(i, &format!("Task {}", i), TaskState::Todo, false);
            manager.add_task(0, task).unwrap();
        }

        assert_eq!(count_all_tasks(&manager), 5);
    }

    #[test]
    fn test_count_all_tasks_with_subtasks() {
        let mut manager = TaskManager::new();

        let task1 = create_test_task(1, "Task 1", TaskState::Todo, false);
        manager.add_task(0, task1).unwrap();

        let task2 = create_test_task(2, "Task 2", TaskState::Todo, false);
        manager.add_task(1, task2).unwrap();

        let task3 = create_test_task(3, "Task 3", TaskState::Todo, false);
        manager.add_task(0, task3).unwrap();

        // Should count all tasks including subtasks
        assert_eq!(count_all_tasks(&manager), 3);
    }

    #[test]
    fn test_count_tasks_by_state_todo() {
        let mut manager = TaskManager::new();

        manager
            .add_task(0, create_test_task(1, "Todo 1", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(0, create_test_task(2, "Todo 2", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(0, create_test_task(3, "Done", TaskState::Completed, false))
            .unwrap();

        assert_eq!(count_tasks_by_state(&manager, "todo"), 2);
    }

    #[test]
    fn test_count_tasks_by_state_completed() {
        let mut manager = TaskManager::new();

        manager
            .add_task(0, create_test_task(1, "Task 1", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(
                0,
                create_test_task(2, "Completed 1", TaskState::Completed, false),
            )
            .unwrap();
        manager
            .add_task(
                0,
                create_test_task(3, "Completed 2", TaskState::Completed, false),
            )
            .unwrap();

        assert_eq!(count_tasks_by_state(&manager, "completed"), 2);
    }

    #[test]
    fn test_count_tasks_by_state_inprogress() {
        let mut manager = TaskManager::new();

        manager
            .add_task(
                0,
                create_test_task(1, "In Progress", TaskState::InProgress, false),
            )
            .unwrap();
        manager
            .add_task(0, create_test_task(2, "Blocked", TaskState::Blocked, false))
            .unwrap();

        assert_eq!(count_tasks_by_state(&manager, "inprogress"), 1);
    }

    #[test]
    fn test_count_pinned_tasks_none() {
        let mut manager = TaskManager::new();

        manager
            .add_task(0, create_test_task(1, "Task 1", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(0, create_test_task(2, "Task 2", TaskState::Todo, false))
            .unwrap();

        assert_eq!(count_pinned_tasks(&manager), 0);
    }

    #[test]
    fn test_count_pinned_tasks_some() {
        let mut manager = TaskManager::new();

        manager
            .add_task(0, create_test_task(1, "Task 1", TaskState::Todo, true))
            .unwrap();
        manager
            .add_task(0, create_test_task(2, "Task 2", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(0, create_test_task(3, "Task 3", TaskState::Todo, true))
            .unwrap();

        assert_eq!(count_pinned_tasks(&manager), 2);
    }

    #[test]
    fn test_get_task_stats() {
        let mut manager = TaskManager::new();

        manager
            .add_task(0, create_test_task(1, "Todo", TaskState::Todo, false))
            .unwrap();
        manager
            .add_task(
                0,
                create_test_task(2, "In Progress", TaskState::InProgress, false),
            )
            .unwrap();
        manager
            .add_task(
                0,
                create_test_task(3, "Completed", TaskState::Completed, false),
            )
            .unwrap();

        let stats = get_task_stats(&manager);

        assert!(stats.contains("Total: 3"));
        assert!(stats.contains("Todo: 1"));
        assert!(stats.contains("In Progress: 1"));
        assert!(stats.contains("Completed: 1"));
    }

    #[test]
    fn test_display_all_tasks_empty() {
        let manager = TaskManager::new();
        // Should print "No tasks. Use 'add' to create a new task."
        // This test just ensures it doesn't panic
        display_all_tasks(&manager);
    }

    #[test]
    fn test_taskbar_file_exists() {
        let path = "test_file_exists.json";

        // File should not exist yet
        assert!(!taskbar_file_exists(path));

        // Create the file
        fs::write(path, "{}").unwrap();

        // File should exist now
        assert!(taskbar_file_exists(path));

        // Clean up
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_and_load_taskbar() {
        let path = "test_save_load.json";

        let mut manager = TaskManager::new();
        let task = create_test_task(1, "Test Task", TaskState::Todo, false);
        manager.add_task(0, task).unwrap();

        // Save the taskbar
        assert!(save_taskbar(path, &manager).is_ok());

        // Load the taskbar
        let loaded = load_taskbar(path).unwrap();
        assert_eq!(count_all_tasks(&loaded), 1);

        // Clean up
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_backup_taskbar() {
        let original_path = "test_original_backup.json";
        let backup_path = "test_backup_copy.json";

        let mut manager = TaskManager::new();
        let task = create_test_task(1, "Backup Test", TaskState::Todo, false);
        manager.add_task(0, task).unwrap();

        // Save the original
        save_taskbar(original_path, &manager).unwrap();

        // Create backup
        assert!(backup_taskbar(original_path, backup_path).is_ok());

        // Verify backup exists
        assert!(taskbar_file_exists(backup_path));

        // Verify backup content matches original
        let original_content = fs::read_to_string(original_path).unwrap();
        let backup_content = fs::read_to_string(backup_path).unwrap();
        assert_eq!(original_content, backup_content);

        // Clean up
        fs::remove_file(original_path).unwrap();
        fs::remove_file(backup_path).unwrap();
    }

    #[test]
    fn test_load_taskbar_fixes_invalid_layers() {
        let path = "test_invalid_layers.json";

        // Create a taskbar with valid structure but manually corrupt the layers
        let mut manager = TaskManager::new();
        let task1 = create_test_task(1, "Task 1", TaskState::Todo, false);
        let task2 = create_test_task(2, "Subtask 1", TaskState::Todo, false);
        let task3 = create_test_task(3, "Nested Subtask", TaskState::Todo, false);

        manager.add_task(0, task1).unwrap();
        manager.add_task(1, task2).unwrap();
        manager.add_task(2, task3).unwrap();

        // Verify correct layers before saving
        // When added to root (layer 0), task1 becomes layer 1
        // When added to task1 (layer 1), task2 becomes layer 2
        // When added to task2 (layer 2), task3 becomes layer 3
        let task1_loaded = manager.root.search_by_id(1).unwrap();
        assert_eq!(task1_loaded.layer, 1);
        let task2_loaded = manager.root.search_by_id(2).unwrap();
        assert_eq!(task2_loaded.layer, 2);
        let task3_loaded = manager.root.search_by_id(3).unwrap();
        assert_eq!(task3_loaded.layer, 3);

        // Save to file
        save_taskbar(path, &manager).unwrap();

        // Manually corrupt the layers in the saved JSON
        let content = fs::read_to_string(path).unwrap();
        let corrupted = content
            .replace("\"layer\":1", "\"layer\":5")
            .replace("\"layer\":2", "\"layer\":0")
            .replace("\"layer\":3", "\"layer\":1");
        fs::write(path, corrupted).unwrap();

        // Load the corrupted taskbar - layers should be fixed
        let mut loaded_manager = load_taskbar(path).unwrap();

        // Verify that layers have been corrected
        let task1_fixed = loaded_manager.root.search_by_id(1).unwrap();
        assert_eq!(
            task1_fixed.layer, 1,
            "Direct subtask of root should have layer 1"
        );

        let task2_fixed = loaded_manager.root.search_by_id(2).unwrap();
        assert_eq!(task2_fixed.layer, 2, "Nested subtask should have layer 2");

        let task3_fixed = loaded_manager.root.search_by_id(3).unwrap();
        assert_eq!(
            task3_fixed.layer, 3,
            "Deeply nested subtask should have layer 3"
        );

        // Clean up
        fs::remove_file(path).unwrap();
    }
}
