use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;

const QUALIFIER: &str = "io";
const ORGANIZATION: &str = "another_taskbar";
const APPLICATION: &str = "another_taskbar";
const GUI_SETTINGS_FILE_NAME: &str = "config.toml";
const TASKBAR_FILE_NAME: &str = "tasks.json";
const THEMES_DIR_NAME: &str = "themes";

fn project_dirs() -> Result<ProjectDirs, String> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or_else(|| "Could not determine application directories for this platform.".to_string())
}

pub fn config_dir() -> Result<PathBuf, String> {
    Ok(project_dirs()?.config_dir().to_path_buf())
}

pub fn data_dir() -> Result<PathBuf, String> {
    Ok(project_dirs()?.data_dir().to_path_buf())
}

pub fn cache_dir() -> Result<PathBuf, String> {
    Ok(project_dirs()?.cache_dir().to_path_buf())
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
    for dir in [config_dir()?, data_dir()?, cache_dir()?, themes_dir()?] {
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Failed to create '{}': {error}", dir.display()))?;
    }

    Ok(())
}

pub fn clear_app_data() -> Result<(), String> {
    let config_dir = config_dir()?;
    let data_dir = data_dir()?;
    let cache_dir = cache_dir()?;

    for dir in [config_dir, data_dir, cache_dir] {
        if dir.exists() {
            fs::remove_dir_all(&dir)
                .map_err(|error| format!("Failed to remove '{}': {error}", dir.display()))?;
        }
    }

    Ok(())
}
