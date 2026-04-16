use std::fs;
use std::path::PathBuf;

#[cfg(all(not(target_os = "android"), feature = "desktop"))]
use directories::BaseDirs;

#[cfg(target_os = "windows")]
const APP_DIR_NAME: &str = "AnotherTaskbar";
#[cfg(not(any(target_os = "windows", target_os = "android")))]
const APP_DIR_NAME: &str = "another_taskbar";
#[cfg(target_os = "android")]
const APP_DIR_NAME: &str = "another_taskbar"; // Mostly unused on Android
const GUI_SETTINGS_FILE_NAME: &str = "config.toml";
const TASKBAR_FILE_NAME: &str = "tasks.json";
const THEMES_DIR_NAME: &str = "themes";
const CACHE_DIR_NAME: &str = "cache";

#[cfg(target_os = "android")]
fn app_dir() -> Result<PathBuf, String> {
    // On Android, try a set of candidate directories and pick the first
    // writable one to avoid startup failures on restricted paths.
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(app_data_dir) = std::env::var("APP_DATA_DIR") {
        if !app_data_dir.trim().is_empty() {
            candidates.push(PathBuf::from(app_data_dir));
        }
    }

    if let Ok(home_dir) = std::env::var("HOME") {
        if !home_dir.trim().is_empty() {
            candidates.push(PathBuf::from(home_dir).join("another_taskbar"));
        }
    }

    if let Ok(tmp_dir) = std::env::var("TMPDIR") {
        if !tmp_dir.trim().is_empty() {
            candidates.push(PathBuf::from(tmp_dir).join("another_taskbar"));
        }
    }

    // Last-resort fallback for Android internal storage layout.
    candidates.push(PathBuf::from(
        "/data/user/0/io.anothertaskbar.mobile/files/another_taskbar",
    ));

    let mut last_error: Option<String> = None;
    for candidate in candidates {
        match fs::create_dir_all(&candidate) {
            Ok(_) => return Ok(candidate),
            Err(error) => {
                last_error = Some(format!("Failed to create '{}': {error}", candidate.display()));
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Failed to resolve writable app dir".to_string()))
}

#[cfg(not(target_os = "android"))]
fn app_dir() -> Result<PathBuf, String> {
    #[cfg(feature = "desktop")]
    {
        let home = BaseDirs::new()
            .ok_or_else(|| "Could not determine user home directory for this platform.".to_string())?
            .home_dir()
            .to_path_buf();
        Ok(home.join(APP_DIR_NAME))
    }
    #[cfg(not(feature = "desktop"))]
    {
        // Fallback for non-desktop builds (shouldn't happen in practice)
        Ok(std::env::current_dir().unwrap_or_default())
    }
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
