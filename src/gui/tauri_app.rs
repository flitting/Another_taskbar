use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
#[cfg(not(target_os = "android"))]
use tauri::image::Image;
#[cfg(not(target_os = "android"))]
use tauri::menu::MenuBuilder;
#[cfg(not(target_os = "android"))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{AppHandle, Manager, State};

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
    notified_task_ids: Mutex<HashSet<u32>>,
    pending_notification_task_id: Mutex<Option<u32>>,
    #[cfg(not(target_os = "android"))]
    tray_alert_active: AtomicBool,
    #[cfg(not(target_os = "android"))]
    tray_flash_worker_running: AtomicBool,
    ignore_all_notifications: AtomicBool,
}

#[cfg(not(target_os = "android"))]
const DEFAULT_TRAY_ICON_BYTES: &[u8] = include_bytes!("../../icons/icon.png");
#[cfg(not(target_os = "android"))]
const NOTIFICATION_TRAY_ICON_BYTES: &[u8] = include_bytes!("../../icons/notification.png");

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
    #[cfg(not(target_os = "android"))]
    {
        use std::process::Command;
        
        #[cfg(not(target_os = "windows"))]
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
                let command = format!(
                    "$target='{quoted_path}'; \
                     for ($i=0; $i -lt 30; $i++) {{ \
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
        if let Err(error) = crate::app_paths::clear_app_data() {
            eprintln!("[cleanup] immediate delete failed, will retry after exit: {error}");
            schedule_delete_after_exit(default_dir.as_path())?;
        }
    }

    #[cfg(target_os = "android")]
    {
        if let Err(error) = crate::app_paths::clear_app_data() {
            eprintln!("[cleanup] failed to delete app data: {error}");
        }
    }

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

#[tauri::command]
fn poll_due_task_notifications(
    app_handle: AppHandle,
    state: State<'_, SharedState>,
    minutes_threshold: Option<i64>,
) -> Result<Vec<crate::tasks::DueTaskNotification>, String> {
    if state.ignore_all_notifications.load(Ordering::SeqCst) {
        return Ok(Vec::new());
    }

    let manager = state
        .manager
        .lock()
        .map_err(|_| "Failed to lock task manager state".to_string())?;

    let threshold = minutes_threshold.unwrap_or(15);
    let upcoming = crate::tasks::find_tasks_due_soon(&manager.root.subtasks, threshold);
    drop(manager);

    let mut sent = Vec::new();
    let mut notified = state
        .notified_task_ids
        .lock()
        .map_err(|_| "Failed to lock notification state".to_string())?;

    for task in upcoming {
        if notified.insert(task.task_id) {
            show_due_notification(&task);
            sent.push(task);
        }
    }

    #[cfg(not(target_os = "android"))]
    if let Some(first_task) = sent.first() {
        let mut pending = state
            .pending_notification_task_id
            .lock()
            .map_err(|_| "Failed to lock pending notification state".to_string())?;
        *pending = Some(first_task.task_id);
        state.tray_alert_active.store(true, Ordering::SeqCst);
        drop(pending);
        if let Err(error) = flash_tray_icon(app_handle.clone()) {
            eprintln!("[tray] failed to start tray flash effect: {error}");
        }
    }
    #[cfg(target_os = "android")]
    {
        let _ = app_handle; // Use it to avoid unused variable warning
    }

    Ok(sent)
}

#[tauri::command]
fn flash_tray_icon(_app_handle: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        // Android doesn't have tray icons
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        let state = _app_handle.state::<SharedState>();
        state.tray_alert_active.store(true, Ordering::SeqCst);
        ensure_tray_flash_worker(_app_handle)
    }
}

#[tauri::command]
fn exit_app(app_handle: AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
fn minimize_to_tray(window: tauri::Window) -> Result<(), String> {
    window
        .hide()
        .map_err(|e| format!("Failed to minimize to tray: {e}"))
}

#[cfg(target_os = "android")]
#[tauri::command]
fn minimize_to_tray(_window: tauri::Window) -> Result<(), String> {
    // On Android, minimize_to_tray does nothing (app goes to background via system)
    Ok(())
}

fn show_main_window(app: &AppHandle) {
    #[cfg(not(target_os = "android"))]
    {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
    
    #[cfg(not(target_os = "android"))]
    {
        clear_tray_notification_alert(app);
        open_pending_summary_from_tray(app);
    }
}

fn ignore_all_notifications(app: &AppHandle) {
    if let Some(state) = app.try_state::<SharedState>() {
        state.ignore_all_notifications.store(true, Ordering::SeqCst);
        #[cfg(not(target_os = "android"))]
        state.tray_alert_active.store(false, Ordering::SeqCst);

        if let Ok(mut pending) = state.pending_notification_task_id.lock() {
            *pending = None;
        }
    }

    #[cfg(not(target_os = "android"))]
    if let Err(error) = set_tray_icon_notification_state(app, false) {
        eprintln!("[tray] failed to reset tray icon while ignoring notifications: {error}");
    }
}

#[cfg(not(target_os = "android"))]
fn load_default_tray_icon() -> Result<Image<'static>, String> {
    Image::from_bytes(DEFAULT_TRAY_ICON_BYTES)
        .or_else(|_| Image::from_bytes(include_bytes!("../../icons/icon.ico")))
        .map_err(|error| {
            format!("Failed to load tray icon from icons/icon.png or icons/icon.ico: {error}")
        })
}

#[cfg(not(target_os = "android"))]
fn load_notification_tray_icon() -> Result<Image<'static>, String> {
    Image::from_bytes(NOTIFICATION_TRAY_ICON_BYTES).map_err(|error| {
        format!("Failed to load tray icon from icons/notification.png: {error}")
    })
}

#[cfg(not(target_os = "android"))]
fn current_tray_id(app: &AppHandle) -> String {
    app.config()
        .app
        .tray_icon
        .as_ref()
        .and_then(|config| config.id.clone())
        .unwrap_or_else(|| "main".into())
}

#[cfg(not(target_os = "android"))]
fn set_tray_icon_for_handle(
    app: &AppHandle,
    icon: Option<Image<'static>>,
) -> Result<(), String> {
    let tray_id = current_tray_id(app);
    let tray = app
        .tray_by_id(&tray_id)
        .ok_or_else(|| format!("Tray with id '{tray_id}' was not found"))?;
    tray.set_icon(icon)
        .map_err(|error| format!("Failed to set tray icon: {error}"))
}

#[cfg(not(target_os = "android"))]
fn set_tray_icon_notification_state(app: &AppHandle, has_notification: bool) -> Result<(), String> {
    let icon = if has_notification {
        Some(load_notification_tray_icon()?)
    } else {
        Some(load_default_tray_icon()?)
    };
    set_tray_icon_for_handle(app, icon)
}

#[cfg(not(target_os = "android"))]
fn clear_tray_notification_alert(app: &AppHandle) {
    if let Some(state) = app.try_state::<SharedState>() {
        state.tray_alert_active.store(false, Ordering::SeqCst);
    }
    if let Err(error) = set_tray_icon_notification_state(app, false) {
        eprintln!("[tray] failed to reset tray icon after opening app: {error}");
    }
}

#[cfg(not(target_os = "android"))]
fn open_pending_summary_from_tray(app: &AppHandle) {
    let Some(state) = app.try_state::<SharedState>() else {
        return;
    };
    let mut pending = match state.pending_notification_task_id.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    let Some(task_id) = *pending else {
        return;
    };
    *pending = None;
    drop(pending);

    if let Some(window) = app.get_webview_window("main") {
        let script = format!("window.handleTrayOpenWithNotification?.({task_id});");
        let _ = window.eval(script.as_str());
    }
}

#[cfg(not(target_os = "android"))]
fn ensure_tray_flash_worker(app_handle: AppHandle) -> Result<(), String> {
    use std::thread;
    use std::time::Duration;

    let state = app_handle.state::<SharedState>();
    if state
        .tray_flash_worker_running
        .swap(true, Ordering::SeqCst)
    {
        return Ok(());
    }

    let default_icon = load_default_tray_icon()?;
    let notification_icon = load_notification_tray_icon()?;
    let handle = app_handle.clone();

    thread::spawn(move || {
        loop {
            let Some(state) = handle.try_state::<SharedState>() else {
                break;
            };

            if !state.tray_alert_active.load(Ordering::SeqCst) {
                break;
            }
            let _ = set_tray_icon_for_handle(&handle, Some(notification_icon.clone()));
            thread::sleep(Duration::from_millis(300));

            if !state.tray_alert_active.load(Ordering::SeqCst) {
                break;
            }
            let _ = set_tray_icon_for_handle(&handle, Some(default_icon.clone()));
            thread::sleep(Duration::from_millis(300));
        }

        if let Some(state) = handle.try_state::<SharedState>() {
            state
                .tray_flash_worker_running
                .store(false, Ordering::SeqCst);
            let alert_active = state.tray_alert_active.load(Ordering::SeqCst);
            let _ = set_tray_icon_notification_state(&handle, alert_active);
        }
    });

    Ok(())
}

#[cfg(target_os = "windows")]
fn show_system_notification(summary: &str, body: &str) -> Result<(), String> {
    let notify_result = notify_rust::Notification::new()
        .appname("Another Taskbar")
        .summary(summary)
        .body(body)
        .show();

    notify_result
        .map(|_| ())
        .map_err(|error| format!("Windows notification failed: {error}"))
}

#[cfg(target_os = "android")]
fn show_system_notification(_summary: &str, _body: &str) -> Result<(), String> {
    // Android notifications are handled via Tauri's built-in notification API
    // For now, we just log that a notification would be sent
    eprintln!("[notification] Android notification: {}", _body);
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "android")))]
fn show_system_notification(_summary: &str, _body: &str) -> Result<(), String> {
    Ok(())
}

fn show_due_notification(task: &crate::tasks::DueTaskNotification) {
    let message = format!(
        "\"{}\" is due in {} minutes.",
        task.task_name, task.minutes_until_due
    );
    if let Err(error) = show_system_notification("Another Taskbar", &message) {
        eprintln!("[notification] failed to show due notification: {error}");
    }
}

pub fn run_gui_app() -> tauri::Result<()> {
    #[cfg(target_os = "linux")]
    prepare_linux_ime_environment();

    let runtime =
        initialize_runtime().map_err(|error| tauri::Error::Io(std::io::Error::other(error)))?;

    #[cfg(not(target_os = "android"))]
    let shared_state = SharedState {
        manager: Mutex::new(runtime.manager),
        taskbar_path: Mutex::new(runtime.taskbar_path),
        settings: Mutex::new(runtime.settings),
        notified_task_ids: Mutex::new(HashSet::new()),
        pending_notification_task_id: Mutex::new(None),
        tray_alert_active: AtomicBool::new(false),
        tray_flash_worker_running: AtomicBool::new(false),
        ignore_all_notifications: AtomicBool::new(false),
    };

    #[cfg(target_os = "android")]
    let shared_state = SharedState {
        manager: Mutex::new(runtime.manager),
        taskbar_path: Mutex::new(runtime.taskbar_path),
        settings: Mutex::new(runtime.settings),
        notified_task_ids: Mutex::new(HashSet::new()),
        pending_notification_task_id: Mutex::new(None),
        ignore_all_notifications: AtomicBool::new(false),
    };

    #[cfg(not(target_os = "android"))]
    let mut builder = tauri::Builder::default()
        .manage(shared_state)
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
            reload_taskbar_file,
            poll_due_task_notifications,
            flash_tray_icon,
            exit_app,
            minimize_to_tray
        ]);

    #[cfg(target_os = "android")]
    let builder = tauri::Builder::default()
        .manage(shared_state)
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
            reload_taskbar_file,
            poll_due_task_notifications,
            flash_tray_icon,
            exit_app,
            minimize_to_tray
        ]);

    #[cfg(not(target_os = "android"))]
    {
        builder = builder
            .setup(|app| {
                let tray_id = current_tray_id(app.handle());

                if let Some(tray) = app.tray_by_id(&tray_id) {
                    let menu = MenuBuilder::new(app)
                        .text("tray_show", "Show Window")
                        .separator()
                        .text("tray_ignore_notifications", "Ignore All Notifications")
                        .separator()
                        .text("tray_quit", "Quit")
                        .build()?;
                    tray.set_menu(Some(menu))?;
                    if let Ok(icon) = load_default_tray_icon() {
                        let _ = tray.set_icon(Some(icon));
                    }
                }

                Ok(())
            })
            .on_menu_event(|app, event| {
                if event.id() == "tray_show" {
                    show_main_window(app);
                } else if event.id() == "tray_ignore_notifications" {
                    ignore_all_notifications(app);
                } else if event.id() == "tray_quit" {
                    app.exit(0);
                }
            })
            .on_tray_icon_event(|app, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    show_main_window(app);
                }
            })
            .on_window_event(|window, event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();

                    // Try to trigger the close confirmation dialog on frontend
                    let handle = window.app_handle().clone();
                    let window_label = window.label().to_string();

                    // Use a webview to call the close handler function
                    std::thread::spawn(move || {
                        // Give the event loop time to process
                        std::thread::sleep(std::time::Duration::from_millis(50));

                        if let Some(w) = handle.get_webview_window(&window_label) {
                            // Call the JavaScript function directly through eval
                            let script = "window.handleCloseRequest?.()";
                            let _ = w.eval(script);
                        }
                    });
                }
            });
    }

    builder.run(tauri::generate_context!())
}
