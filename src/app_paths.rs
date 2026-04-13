use std::fs;
use std::path::PathBuf;

use directories::BaseDirs;

#[cfg(target_os = "windows")]
const APP_DIR_NAME: &str = "AnotherTaskbar";
#[cfg(not(target_os = "windows"))]
const APP_DIR_NAME: &str = "another_taskbar";
const GUI_SETTINGS_FILE_NAME: &str = "config.toml";
const TASKBAR_FILE_NAME: &str = "tasks.json";
const THEMES_DIR_NAME: &str = "themes";
const CACHE_DIR_NAME: &str = "cache";

fn app_dir() -> Result<PathBuf, String> {
    let home = BaseDirs::new()
        .ok_or_else(|| "Could not determine user home directory for this platform.".to_string())?
        .home_dir()
        .to_path_buf();
    Ok(home.join(APP_DIR_NAME))
}

pub fn config_dir() -> Result<PathBuf, String> {
    app_dir()
}

pub fn data_dir() -> Result<PathBuf, String> {
    app_dir()
}

pub fn cache_dir() -> Result<PathBuf, String> {
    Ok(app_dir()?.join(CACHE_DIR_NAME))
}

pub fn gui_settings_path() -> Result<PathBuf, String> {
    Ok(config_dir()?.join(GUI_SETTINGS_FILE_NAME))
}

pub fn taskbar_path() -> Result<PathBuf, String> {
    Ok(data_dir()?.join(TASKBAR_FILE_NAME))
}

pub fn themes_dir() -> Result<PathBuf, String> {
    Ok(config_dir()?.join(THEMES_DIR_NAME))
}

pub fn ensure_app_dirs() -> Result<(), String> {
    for dir in [config_dir()?, cache_dir()?, themes_dir()?] {
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Failed to create '{}': {error}", dir.display()))?;
    }

    Ok(())
}

pub fn clear_app_data() -> Result<(), String> {
    let root_dir = app_dir()?;

    if root_dir.exists() {
        fs::remove_dir_all(&root_dir)
            .map_err(|error| format!("Failed to remove '{}': {error}", root_dir.display()))?;
    }

    Ok(())
}
