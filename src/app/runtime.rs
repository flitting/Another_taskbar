use std::path::{Path, PathBuf};

use crate::bootstrap::initialize_app_storage;
use crate::files::{load_taskbar, save_taskbar, TaskbarDefaultPath, DEFAULT_TASKBAR_FILE_NAME};
use crate::gui::settings::{load_gui_settings, GuiSettings};
use crate::locale::set_current_language;
use crate::tasks::{TaskDraft, TaskImportance, TaskManager, TaskSortMode, TaskState, TaskUrgency};

pub struct AppRuntime {
    pub taskbar_path: PathBuf,
    pub manager: TaskManager,
    pub settings: GuiSettings,
}

pub fn default_taskbar_path() -> PathBuf {
    TaskbarDefaultPath::resolve().unwrap_or_else(|_| PathBuf::from(DEFAULT_TASKBAR_FILE_NAME))
}

pub fn taskbar_path_from_settings(settings: &GuiSettings) -> PathBuf {
    let configured = settings.task_data_directory.trim();
    let base = if configured.is_empty() {
        default_taskbar_path()
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        PathBuf::from(configured)
    };

    base.join(DEFAULT_TASKBAR_FILE_NAME)
}

pub fn load_or_initialize_manager(path: &Path) -> TaskManager {
    match load_taskbar(path) {
        Ok(manager) => manager,
        Err(_) => TaskManager::new(),
    }
}

pub fn persist_manager(path: &Path, manager: &TaskManager) -> Result<(), String> {
    save_taskbar(path, manager)
}

fn seeded_task_manager() -> TaskManager {
    let mut manager = TaskManager::new();
    let add = |manager: &mut TaskManager,
               parent_id: u32,
               name: &str,
               description: &str,
               state: TaskState,
               urgency: Option<TaskUrgency>,
               importance: Option<TaskImportance>,
               pinned: bool|
     -> u32 {
        manager
            .create_task_from_draft(
                parent_id,
                TaskDraft {
                    name: name.to_string(),
                    description: description.to_string(),
                    state,
                    urgency,
                    importance,
                    tags: vec!["example".to_string()],
                    pinned,
                    due_date: None,
                    completed_at: None,
                    recurrence: None,
                },
            )
            .unwrap_or(0)
    };

    let planning_id = add(
        &mut manager,
        0,
        "Plan imaginary launch",
        "Parent sample task with nested subtasks.",
        TaskState::Todo,
        Some(TaskUrgency::High),
        Some(TaskImportance::High),
        true,
    );
    if planning_id != 0 {
        let draft_id = add(
            &mut manager,
            planning_id,
            "Draft random notes",
            "Sample child task under parent.",
            TaskState::Todo,
            Some(TaskUrgency::Low),
            None,
            false,
        );
        if draft_id != 0 {
            let _ = add(
                &mut manager,
                draft_id,
                "Archive fake outcome",
                "Grandchild sample task for hierarchy check.",
                TaskState::Completed,
                None,
                Some(TaskImportance::Low),
                false,
            );
        }
        let _ = add(
            &mut manager,
            planning_id,
            "Tick done item",
            "Completed child task sample.",
            TaskState::Completed,
            None,
            None,
            false,
        );
    }

    let _ = add(
        &mut manager,
        0,
        "Collect invisible stickers",
        "Standalone sample task.",
        TaskState::Todo,
        None,
        Some(TaskImportance::High),
        false,
    );
    let _ = add(
        &mut manager,
        0,
        "Write nonsense checklist",
        "Another standalone sample task.",
        TaskState::Completed,
        Some(TaskUrgency::High),
        None,
        true,
    );
    let _ = add(
        &mut manager,
        0,
        "Rename or delete these examples",
        "Safe to edit immediately.",
        TaskState::Todo,
        None,
        None,
        false,
    );

    manager.sort_for_mode(&TaskSortMode::Custom);
    manager
}

pub fn ensure_taskbar_file(path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    let manager = seeded_task_manager();
    save_taskbar(path, &manager)
}

pub fn initialize_runtime() -> Result<AppRuntime, String> {
    initialize_app_storage()?;
    let settings = load_gui_settings();
    let taskbar_path = taskbar_path_from_settings(&settings);
    ensure_taskbar_file(taskbar_path.as_path())?;
    let mut manager = load_or_initialize_manager(taskbar_path.as_path());
    if manager.root.subtasks.is_empty() {
        manager = seeded_task_manager();
        save_taskbar(taskbar_path.as_path(), &manager)?;
    }
    set_current_language(settings.selected_language);

    Ok(AppRuntime {
        taskbar_path,
        manager,
        settings,
    })
}
