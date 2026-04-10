use crate::app_paths;
use crate::files::{save_taskbar, TaskbarDefaultPath};
use crate::gui::settings::{save_gui_settings, GuiSettings};
use crate::tasks::TaskManager;

pub fn initialize_app_storage() -> Result<(), String> {
    app_paths::ensure_app_dirs()?;

    let settings_path = app_paths::gui_settings_path()?;
    if !settings_path.exists() {
        save_gui_settings(&GuiSettings::default())?;
    }

    let taskbar_path = TaskbarDefaultPath::resolve()?;
    if !taskbar_path.exists() {
        save_taskbar(&taskbar_path, &TaskManager::new())?;
    }

    Ok(())
}
