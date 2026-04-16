use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app_paths;
use crate::locale::AppLanguage;
use crate::tasks::{TaskSortMode, TaskState};

const DEFAULT_THEME_NAME: &str = "dark";
const LIGHT_THEME_NAME: &str = "light";
pub const DEFAULT_TASK_FONT_SIZE: u16 = 14;
pub const DEFAULT_LANGUAGE: AppLanguage = AppLanguage::English;

const DARK_THEME_TOML: &str = include_str!("../../themes/dark.toml");
const LIGHT_THEME_TOML: &str = include_str!("../../themes/light.toml");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    MinimizeToTray,
    ExitApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiSettings {
    pub selected_theme: String,
    #[serde(default = "default_task_font_size")]
    pub task_font_size: u16,
    #[serde(default = "default_language")]
    pub selected_language: AppLanguage,
    #[serde(default = "default_task_sort_mode")]
    pub task_sort_mode: TaskSortMode,
    #[serde(default)]
    pub enabled_optional_states: Vec<TaskState>,
    #[serde(default = "default_auto_complete_parent_tasks")]
    pub auto_complete_parent_tasks: bool,
    #[serde(default = "default_task_data_directory")]
    pub task_data_directory: String,
    #[serde(default = "default_ui_scale")]
    pub ui_scale: f32,
    #[serde(default = "default_remember_close_action")]
    pub remember_close_action: bool,
    #[serde(default = "default_close_action")]
    pub remembered_close_action: CloseAction,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self {
            selected_theme: DEFAULT_THEME_NAME.to_string(),
            task_font_size: default_task_font_size(),
            selected_language: default_language(),
            task_sort_mode: default_task_sort_mode(),
            enabled_optional_states: Vec::new(),
            auto_complete_parent_tasks: default_auto_complete_parent_tasks(),
            task_data_directory: default_task_data_directory(),
            ui_scale: default_ui_scale(),
            remember_close_action: default_remember_close_action(),
            remembered_close_action: default_close_action(),
        }
    }
}

fn default_task_font_size() -> u16 {
    DEFAULT_TASK_FONT_SIZE
}

fn default_language() -> AppLanguage {
    DEFAULT_LANGUAGE
}

fn default_task_sort_mode() -> TaskSortMode {
    TaskSortMode::Custom
}

fn default_auto_complete_parent_tasks() -> bool {
    true
}

fn default_task_data_directory() -> String {
    app_paths::data_dir()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn default_ui_scale() -> f32 {
    1.0
}

fn default_remember_close_action() -> bool {
    false
}

fn default_close_action() -> CloseAction {
    CloseAction::MinimizeToTray
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePalette {
    pub name: String,
    pub primary_bg: String,
    pub secondary_bg: String,
    pub tertiary_bg: String,
    pub pinned_bg: String,
    pub accent_color: String,
    pub highlight_bg: String,
    pub selection_bg: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub menu_bg: String,
    pub tooltip_bg: String,
    pub tag_bg: String,
    pub tag_active_bg: String,
    pub input_bg: String,
    pub todo_color: String,
    pub in_progress_color: String,
    pub blocked_color: String,
    pub completed_color: String,
    pub archived_color: String,
    pub importance_high_stripe: String,
    pub importance_low_stripe: String,
    pub urgency_high_stripe: String,
    pub urgency_low_stripe: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ThemePaletteFile {
    name: Option<String>,
    primary_bg: String,
    secondary_bg: String,
    tertiary_bg: String,
    pinned_bg: String,
    accent_color: String,
    highlight_bg: String,
    selection_bg: String,
    text_primary: String,
    text_secondary: String,
    text_muted: String,
    menu_bg: String,
    tooltip_bg: String,
    tag_bg: String,
    tag_active_bg: String,
    input_bg: String,
    todo_color: String,
    in_progress_color: String,
    blocked_color: String,
    completed_color: String,
    archived_color: String,
    importance_high_stripe: String,
    importance_low_stripe: String,
    urgency_high_stripe: String,
    urgency_low_stripe: String,
}

impl ThemePaletteFile {
    fn into_palette(self, fallback_name: &str) -> Result<ThemePalette, String> {
        Ok(ThemePalette {
            name: self.name.unwrap_or_else(|| fallback_name.to_string()),
            primary_bg: normalize_hex_color(&self.primary_bg)?,
            secondary_bg: normalize_hex_color(&self.secondary_bg)?,
            tertiary_bg: normalize_hex_color(&self.tertiary_bg)?,
            pinned_bg: normalize_hex_color(&self.pinned_bg)?,
            accent_color: normalize_hex_color(&self.accent_color)?,
            highlight_bg: normalize_hex_color(&self.highlight_bg)?,
            selection_bg: normalize_hex_color(&self.selection_bg)?,
            text_primary: normalize_hex_color(&self.text_primary)?,
            text_secondary: normalize_hex_color(&self.text_secondary)?,
            text_muted: normalize_hex_color(&self.text_muted)?,
            menu_bg: normalize_hex_color(&self.menu_bg)?,
            tooltip_bg: normalize_hex_color(&self.tooltip_bg)?,
            tag_bg: normalize_hex_color(&self.tag_bg)?,
            tag_active_bg: normalize_hex_color(&self.tag_active_bg)?,
            input_bg: normalize_hex_color(&self.input_bg)?,
            todo_color: normalize_hex_color(&self.todo_color)?,
            in_progress_color: normalize_hex_color(&self.in_progress_color)?,
            blocked_color: normalize_hex_color(&self.blocked_color)?,
            completed_color: normalize_hex_color(&self.completed_color)?,
            archived_color: normalize_hex_color(&self.archived_color)?,
            importance_high_stripe: normalize_hex_color(&self.importance_high_stripe)?,
            importance_low_stripe: normalize_hex_color(&self.importance_low_stripe)?,
            urgency_high_stripe: normalize_hex_color(&self.urgency_high_stripe)?,
            urgency_low_stripe: normalize_hex_color(&self.urgency_low_stripe)?,
        })
    }
}

pub fn initialize_theme_files() -> Result<(), String> {
    fs::create_dir_all(custom_themes_dir()?)
        .map_err(|error| format!("Failed to create themes dir: {error}"))?;
    Ok(())
}

pub fn load_gui_settings() -> GuiSettings {
    let path = match gui_settings_path() {
        Ok(path) => path,
        Err(_) => return GuiSettings::default(),
    };

    match fs::read_to_string(&path) {
        Ok(content) => {
            let mut settings = toml::from_str::<GuiSettings>(&content).unwrap_or_default();
            normalize_gui_settings(&mut settings);
            settings
        }
        Err(_) => {
            let settings = GuiSettings::default();
            let _ = save_gui_settings(&settings);
            settings
        }
    }
}

pub fn save_gui_settings(settings: &GuiSettings) -> Result<(), String> {
    let mut normalized = settings.clone();
    normalize_gui_settings(&mut normalized);

    let content = toml::to_string_pretty(&normalized)
        .map_err(|error| format!("Failed to serialize settings: {error}"))?;
    let path = gui_settings_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create config dir: {error}"))?;
    }
    fs::write(path, content).map_err(|error| format!("Failed to save settings: {error}"))
}

fn normalize_gui_settings(settings: &mut GuiSettings) {
    settings.task_font_size = settings.task_font_size.clamp(11, 28);
    settings.enabled_optional_states =
        normalize_optional_states(settings.enabled_optional_states.clone());
    let trimmed = settings.task_data_directory.trim();
    settings.task_data_directory = if trimmed.is_empty() {
        default_task_data_directory()
    } else {
        trimmed.to_string()
    };
    settings.ui_scale = settings.ui_scale.clamp(0.8, 1.4);
}

fn normalize_optional_states(states: Vec<TaskState>) -> Vec<TaskState> {
    let mut normalized = Vec::new();
    for state in states {
        if !matches!(
            state,
            TaskState::InProgress | TaskState::Blocked | TaskState::Archived
        ) {
            continue;
        }
        if normalized.iter().any(|existing| existing == &state) {
            continue;
        }
        normalized.push(state);
    }
    normalized
}

pub fn available_theme_names() -> Result<Vec<String>, String> {
    initialize_theme_files()?;

    let mut names = vec![DEFAULT_THEME_NAME.to_string(), LIGHT_THEME_NAME.to_string()];
    let entries = fs::read_dir(custom_themes_dir()?)
        .map_err(|error| format!("Failed to read themes dir: {error}"))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("Failed to read theme entry: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }

        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            names.push(stem.to_string());
        }
    }

    names.sort();
    names.dedup();

    if names.is_empty() {
        names.push(DEFAULT_THEME_NAME.to_string());
    }

    Ok(names)
}

pub fn load_theme_palette(theme_name: &str) -> Result<ThemePalette, String> {
    initialize_theme_files()?;

    match theme_name {
        DEFAULT_THEME_NAME => load_theme_palette_from_str(DARK_THEME_TOML, DEFAULT_THEME_NAME),
        LIGHT_THEME_NAME => load_theme_palette_from_str(LIGHT_THEME_TOML, LIGHT_THEME_NAME),
        _ => {
            let path = theme_path(theme_name)?;
            load_theme_palette_from_path(&path, theme_name)
        }
    }
}

pub fn load_theme_palette_from_path(
    path: &Path,
    fallback_name: &str,
) -> Result<ThemePalette, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read theme '{}': {error}", path.display()))?;
    let file: ThemePaletteFile = toml::from_str(&content)
        .map_err(|error| format!("Failed to parse theme '{}': {error}", path.display()))?;

    file.into_palette(fallback_name)
}

pub fn load_theme_palette_from_str(
    content: &str,
    fallback_name: &str,
) -> Result<ThemePalette, String> {
    let file: ThemePaletteFile = toml::from_str(content)
        .map_err(|error| format!("Failed to parse built-in theme '{fallback_name}': {error}"))?;

    file.into_palette(fallback_name)
}

pub fn import_theme_file(path: &Path) -> Result<String, String> {
    initialize_theme_files()?;

    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Invalid theme filename: {}", path.display()))?;
    let target = theme_path(stem)?;

    let content = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read theme '{}': {error}", path.display()))?;
    load_theme_palette_from_path(path, stem)?;
    fs::write(&target, content).map_err(|error| {
        format!(
            "Failed to save imported theme '{}': {error}",
            target.display()
        )
    })?;

    Ok(stem.to_string())
}

pub fn apply_saved_theme() -> Result<ThemePalette, String> {
    let settings = load_gui_settings();
    load_theme_palette(&settings.selected_theme).or_else(|_| load_theme_palette(DEFAULT_THEME_NAME))
}

pub fn theme_path(theme_name: &str) -> Result<PathBuf, String> {
    Ok(custom_themes_dir()?.join(format!("{theme_name}.toml")))
}

pub fn custom_themes_dir() -> Result<PathBuf, String> {
    app_paths::themes_dir()
}

pub fn gui_settings_path() -> Result<PathBuf, String> {
    app_paths::gui_settings_path()
}

fn normalize_hex_color(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);

    if hex.len() != 6 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!(
            "Invalid color '{input}': expected 6 hex digits (example: #AABBCC)"
        ));
    }

    Ok(format!("#{}", hex.to_ascii_uppercase()))
}
