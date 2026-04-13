use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::app::runtime::{
    ensure_taskbar_file, initialize_runtime, load_or_initialize_manager, persist_manager,
    taskbar_path_from_settings,
};
use crate::files::load_taskbar;
use crate::gui::settings::{
    apply_saved_theme, available_theme_names, import_theme_file, load_theme_palette,
    save_gui_settings, GuiSettings, ThemePalette,
};
use crate::locale::{all_strings_for, set_current_language, text_for, AppLanguage};
use crate::tasks::{Task, TaskDraft, TaskManager, TaskSortMode, TaskState};

struct SharedState {
    manager: Mutex<TaskManager>,
    taskbar_path: Mutex<PathBuf>,
    settings: Mutex<GuiSettings>,
}

#[derive(Debug, Serialize)]
struct AppSnapshot {
    tasks: Vec<Task>,
    settings: GuiSettings,
    strings: std::collections::HashMap<String, String>,
    available_languages: Vec<LanguageOption>,
    available_themes: Vec<String>,
    common_tags: Vec<String>,
    active_theme: ThemePalette,
    can_undo: bool,
    theme_dir_path: String,
}

#[derive(Debug, Deserialize)]
struct ThemeSelection {
    theme_name: String,
}

#[derive(Debug, Serialize)]
struct LanguageOption {
    code: String,
    label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MoveRelation {
    Before,
    After,
    AsSubtask,
    AppendRoot,
}

fn current_theme_or_default() -> Result<ThemePalette, String> {
    apply_saved_theme().or_else(|_| load_theme_palette("dark"))
}

fn current_taskbar_path(state: &State<'_, SharedState>) -> Result<PathBuf, String> {
    state
        .taskbar_path
        .lock()
        .map_err(|_| "Failed to lock taskbar path state".to_string())
        .map(|path| path.clone())
}

fn persist_current_manager(
    state: &State<'_, SharedState>,
    manager: &TaskManager,
) -> Result<(), String> {
    let path = current_taskbar_path(state)?;
    persist_manager(path.as_path(), manager)
}

#[cfg(target_os = "linux")]
fn prepare_linux_ime_environment() {
    use std::env;

    if env::var_os("GTK_IM_MODULE").is_some() {
        return;
    }

    let xmods = env::var("XMODIFIERS").unwrap_or_default().to_lowercase();
    let qt_im = env::var("QT_IM_MODULE").unwrap_or_default().to_lowercase();

    let guessed = if xmods.contains("fcitx") || qt_im.contains("fcitx") {
        Some("fcitx")
    } else if xmods.contains("ibus") || qt_im.contains("ibus") {
        Some("ibus")
    } else {
        None
    };

    if let Some(module) = guessed {
        env::set_var("GTK_IM_MODULE", module);
    }
}

#[tauri::command]
fn load_app_state(state: State<'_, SharedState>) -> Result<AppSnapshot, String> {
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;
    let settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?;

    let selected_language = settings.selected_language;
    manager.apply_recurring_updates();
    if settings.auto_complete_parent_tasks {
        manager.apply_parent_completion_rollups();
    }
    manager.sort_for_mode(&settings.task_sort_mode);
    persist_current_manager(&state, &manager)?;
    set_current_language(selected_language);

    Ok(AppSnapshot {
        tasks: manager.root.subtasks.clone(),
        settings: settings.clone(),
        strings: all_strings_for(selected_language),
        available_languages: AppLanguage::all()
            .into_iter()
            .map(|language| LanguageOption {
                code: language.code().to_string(),
                label: text_for(
                    selected_language,
                    match language {
                        AppLanguage::English => "language_english",
                        AppLanguage::ChineseSimplified => "language_chinese",
                    },
                ),
            })
            .collect(),
        available_themes: available_theme_names().unwrap_or_else(|_| vec!["dark".to_string()]),
        common_tags: manager.most_common_tags(16),
        active_theme: current_theme_or_default()?,
        can_undo: manager.can_undo(),
        theme_dir_path: crate::app_paths::themes_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_default(),
    })
}

#[tauri::command]
fn move_task(
    state: State<'_, SharedState>,
    task_id: u32,
    target_id: u32,
    relation: MoveRelation,
) -> Result<(), String> {
    let auto_complete_parent_tasks = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .auto_complete_parent_tasks;
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    match relation {
        MoveRelation::Before => manager.move_task_before(task_id, target_id)?,
        MoveRelation::After => manager.move_task_after(task_id, target_id)?,
        MoveRelation::AsSubtask => manager.move_task_as_subtask(task_id, target_id)?,
        MoveRelation::AppendRoot => manager.move_task_as_subtask(task_id, 0)?,
    }

    let mut shared_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?;
    shared_settings.task_sort_mode = TaskSortMode::Custom;
    save_gui_settings(&shared_settings)?;
    if auto_complete_parent_tasks {
        manager.apply_parent_completion_rollups();
    }
    manager.sort_for_mode(&TaskSortMode::Custom);

    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn create_task(
    state: State<'_, SharedState>,
    parent_id: u32,
    draft: TaskDraft,
) -> Result<u32, String> {
    let sort_mode = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .task_sort_mode
        .clone();
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    let id = manager.create_task_from_draft(parent_id, draft)?;
    manager.sort_for_mode(&sort_mode);
    persist_current_manager(&state, &manager)?;
    Ok(id)
}

#[tauri::command]
fn update_task(state: State<'_, SharedState>, id: u32, draft: TaskDraft) -> Result<(), String> {
    let sort_mode = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .task_sort_mode
        .clone();
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.update_task_from_draft(id, draft)?;
    manager.sort_for_mode(&sort_mode);
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn delete_task(state: State<'_, SharedState>, id: u32) -> Result<(), String> {
    let auto_complete_parent_tasks = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .auto_complete_parent_tasks;
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.delete_task(id)?;
    if auto_complete_parent_tasks {
        manager.apply_parent_completion_rollups();
    }
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn toggle_task_pinned(state: State<'_, SharedState>, id: u32) -> Result<(), String> {
    let sort_mode = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .task_sort_mode
        .clone();
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.toggle_task_pinned(id)?;
    manager.sort_for_mode(&sort_mode);
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn set_task_state(
    state: State<'_, SharedState>,
    id: u32,
    task_state: TaskState,
    cascade_descendants: Option<bool>,
) -> Result<(), String> {
    let settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .clone();
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.set_task_state_with_options(
        id,
        task_state,
        cascade_descendants.unwrap_or(false),
        settings.auto_complete_parent_tasks,
    )?;
    manager.sort_for_mode(&settings.task_sort_mode);
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn update_task_with_options(
    state: State<'_, SharedState>,
    id: u32,
    draft: TaskDraft,
    cascade_descendants: Option<bool>,
) -> Result<(), String> {
    let settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .clone();
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.update_task_from_draft_with_options(
        id,
        draft,
        cascade_descendants.unwrap_or(false),
        settings.auto_complete_parent_tasks,
    )?;
    manager.sort_for_mode(&settings.task_sort_mode);
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn clear_all_tasks(state: State<'_, SharedState>) -> Result<(), String> {
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.clear_tasks();
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn undo_last_change(state: State<'_, SharedState>) -> Result<(), String> {
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    manager.undo_last_change()?;
    persist_current_manager(&state, &manager)
}

#[tauri::command]
fn save_gui_settings_cmd(
    state: State<'_, SharedState>,
    settings: GuiSettings,
) -> Result<GuiSettings, String> {
    let previous_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?
        .clone();
    save_gui_settings(&settings)?;
    let persisted_settings = crate::gui::settings::load_gui_settings();
    let previous_path = taskbar_path_from_settings(&previous_settings);
    let new_path = taskbar_path_from_settings(&persisted_settings);

    {
        let mut manager = state
            .manager
            .lock()
            .map_err(|_| "Failed to lock task manager state".to_string())?;
        if previous_path != new_path {
            ensure_taskbar_file(new_path.as_path())?;
            *manager = load_or_initialize_manager(new_path.as_path());
        }
        if persisted_settings.auto_complete_parent_tasks {
            manager.apply_parent_completion_rollups();
        }
        manager.sort_for_mode(&persisted_settings.task_sort_mode);
    }

    {
        let mut shared_path = state
            .taskbar_path
            .lock()
            .map_err(|_| "Failed to lock taskbar path state".to_string())?;
        *shared_path = new_path;
    }
    {
        let manager = state
            .manager
            .lock()
            .map_err(|_| "Failed to lock task manager state".to_string())?;
        persist_current_manager(&state, &manager)?;
    }
    let mut shared_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?;
    *shared_settings = persisted_settings.clone();
    set_current_language(persisted_settings.selected_language);

    Ok(persisted_settings)
}

#[tauri::command]
fn set_theme(
    state: State<'_, SharedState>,
    payload: ThemeSelection,
) -> Result<ThemePalette, String> {
    let palette = load_theme_palette(&payload.theme_name)?;

    let mut shared_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?;
    shared_settings.selected_theme = payload.theme_name;
    save_gui_settings(&shared_settings)?;

    Ok(palette)
}

#[tauri::command]
fn import_theme_file_cmd(state: State<'_, SharedState>, path: String) -> Result<String, String> {
    let theme_name = import_theme_file(Path::new(&path))?;

    let mut shared_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock GUI settings state".to_string())?;
    shared_settings.selected_theme = theme_name.clone();
    save_gui_settings(&shared_settings)?;

    Ok(theme_name)
}

#[tauri::command]
fn delete_all_data_and_exit(app_handle: AppHandle) -> Result<(), String> {
    fn shell_quote_posix(value: &str) -> String {
        let escaped = value.replace('\'', r"'\''");
        format!("'{escaped}'")
    }

    #[cfg(target_os = "windows")]
    fn shell_quote_powershell(value: &str) -> String {
        value.replace('\'', "''")
    }

    fn schedule_delete_after_exit(path: &Path) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();
        if path_str.trim().is_empty() {
            return Err("Default data directory path is empty.".to_string());
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs::OpenOptions;
            use std::os::unix::process::CommandExt;
            use std::process::Stdio;

            let parent_pid = std::process::id();
            let quoted_path = shell_quote_posix(&path_str);

            let command = format!(
                "while kill -0 {parent_pid} 2>/dev/null; do sleep 0.2; done; \
                 sleep 0.8; \
                 i=0; \
                 while [ $i -lt 20 ]; do \
                   if [ ! -e {quoted_path} ]; then exit 0; fi; \
                   rm -rf -- {quoted_path}; \
                   if [ ! -e {quoted_path} ]; then exit 0; fi; \
                   i=$((i+1)); \
                   sleep 0.5; \
                 done; \
                 exit 1"
            );

            let devnull_in = OpenOptions::new()
                .read(true)
                .open("/dev/null")
                .map_err(|e| format!("Failed to open /dev/null for stdin: {e}"))?;
            let devnull_out = OpenOptions::new()
                .write(true)
                .open("/dev/null")
                .map_err(|e| format!("Failed to open /dev/null for stdout: {e}"))?;
            let devnull_err = OpenOptions::new()
                .write(true)
                .open("/dev/null")
                .map_err(|e| format!("Failed to open /dev/null for stderr: {e}"))?;

            let mut cmd = Command::new("sh");
            cmd.args(["-c", &command])
                .stdin(Stdio::from(devnull_in))
                .stdout(Stdio::from(devnull_out))
                .stderr(Stdio::from(devnull_err));

            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }

            cmd.spawn()
                .map_err(|error| format!("Failed to schedule cleanup: {error}"))?;
        }

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;

            const CREATE_NO_WINDOW: u32 = 0x08000000;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

            let quoted_path = shell_quote_powershell(&path_str);
            let parent_pid = std::process::id();

            let command = format!(
                "$pidToWait={parent_pid}; \
                 while (Get-Process -Id $pidToWait -ErrorAction SilentlyContinue) {{ Start-Sleep -Milliseconds 300 }}; \
                 Start-Sleep -Milliseconds 800; \
                 $target='{quoted_path}'; \
                 for ($i=0; $i -lt 20; $i++) {{ \
                     if (-not (Test-Path -LiteralPath $target)) {{ exit 0 }}; \
                     Remove-Item -LiteralPath $target -Recurse -Force -ErrorAction SilentlyContinue; \
                     if (-not (Test-Path -LiteralPath $target)) {{ exit 0 }}; \
                     Start-Sleep -Milliseconds 500; \
                 }}; \
                 exit 1"
            );

            Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    &command,
                ])
                .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP)
                .spawn()
                .map_err(|error| format!("Failed to schedule Windows cleanup: {error}"))?;
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let parent_pid = std::process::id();
            let quoted_path = shell_quote_posix(&path_str);
            let command = format!(
                "while kill -0 {parent_pid} 2>/dev/null; do sleep 0.2; done; \
                 sleep 0.8; \
                 rm -rf -- {quoted_path}"
            );

            Command::new("sh")
                .args(["-c", &command])
                .spawn()
                .map_err(|error| format!("Failed to schedule cleanup: {error}"))?;
        }

        Ok(())
    }

    let default_dir = crate::app_paths::data_dir()?;
    schedule_delete_after_exit(default_dir.as_path())?;
    app_handle.exit(0);
    Ok(())
}
#[tauri::command]
fn reload_taskbar_file(state: State<'_, SharedState>) -> Result<(), String> {
    let path = current_taskbar_path(&state)?;
    let loaded = load_taskbar(path.as_path())?;
    let mut manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;
    *manager = loaded;
    Ok(())
}

pub fn run_gui_app() -> tauri::Result<()> {
    #[cfg(target_os = "linux")]
    prepare_linux_ime_environment();

    let runtime =
        initialize_runtime().map_err(|error| tauri::Error::Io(std::io::Error::other(error)))?;

    tauri::Builder::default()
        .manage(SharedState {
            manager: Mutex::new(runtime.manager),
            taskbar_path: Mutex::new(runtime.taskbar_path),
            settings: Mutex::new(runtime.settings),
        })
        .invoke_handler(tauri::generate_handler![
            load_app_state,
            create_task,
            update_task,
            update_task_with_options,
            delete_task,
            toggle_task_pinned,
            set_task_state,
            move_task,
            clear_all_tasks,
            undo_last_change,
            save_gui_settings_cmd,
            set_theme,
            import_theme_file_cmd,
            delete_all_data_and_exit,
            reload_taskbar_file
        ])
        .run(tauri::generate_context!())
}
