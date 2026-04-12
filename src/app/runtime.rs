use std::path::{Path, PathBuf};

use crate::bootstrap::initialize_app_storage;
use crate::files::{load_taskbar, save_taskbar, TaskbarDefaultPath, DEFAULT_TASKBAR_FILE_NAME};
use crate::gui::settings::{load_gui_settings, GuiSettings};
use crate::locale::set_current_language;
use crate::tasks::TaskManager;

pub struct AppRuntime {
    pub taskbar_path: PathBuf,
    pub manager: TaskManager,
    pub settings: GuiSettings,
}

pub fn default_taskbar_path() -> PathBuf {
    TaskbarDefaultPath::resolve().unwrap_or_else(|_| PathBuf::from(DEFAULT_TASKBAR_FILE_NAME))
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

pub fn initialize_runtime() -> Result<AppRuntime, String> {
    initialize_app_storage()?;
    let taskbar_path = default_taskbar_path();
    let manager = load_or_initialize_manager(taskbar_path.as_path());
    let settings = load_gui_settings();
    set_current_language(settings.selected_language);

    Ok(AppRuntime {
        taskbar_path,
        manager,
        settings,
    })
}
