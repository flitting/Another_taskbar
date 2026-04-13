use crate::app_paths;
use crate::gui::settings::{save_gui_settings, GuiSettings};

pub fn initialize_app_storage() -> Result<(), String> {
    app_paths::ensure_app_dirs()?;

    let settings_path = app_paths::gui_settings_path()?;
    if !settings_path.exists() {
        save_gui_settings(&GuiSettings::default())?;
    }

    Ok(())
}
