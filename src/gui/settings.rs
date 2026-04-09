use std::fs;
use std::path::{Path, PathBuf};

use iced::Color;
use serde::{Deserialize, Serialize};

use super::theme::{apply_theme_palette, ThemePalette};

pub const THEMES_DIR: &str = "themes";
pub const GUI_SETTINGS_PATH: &str = "settings.toml";
const DEFAULT_THEME_NAME: &str = "dark";

const DARK_THEME_TOML: &str = include_str!("../../themes/dark.toml");
const LIGHT_THEME_TOML: &str = include_str!("../../themes/light.toml");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiSettings {
    pub selected_theme: String,
    #[serde(default = "default_show_details_aside")]
    pub show_details_aside: bool,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self {
            selected_theme: DEFAULT_THEME_NAME.to_string(),
            show_details_aside: default_show_details_aside(),
        }
    }
}

fn default_show_details_aside() -> bool {
    true
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
            primary_bg: parse_hex_color(&self.primary_bg)?,
            secondary_bg: parse_hex_color(&self.secondary_bg)?,
            tertiary_bg: parse_hex_color(&self.tertiary_bg)?,
            pinned_bg: parse_hex_color(&self.pinned_bg)?,
            accent_color: parse_hex_color(&self.accent_color)?,
            highlight_bg: parse_hex_color(&self.highlight_bg)?,
            selection_bg: parse_hex_color(&self.selection_bg)?,
            text_primary: parse_hex_color(&self.text_primary)?,
            text_secondary: parse_hex_color(&self.text_secondary)?,
            text_muted: parse_hex_color(&self.text_muted)?,
            menu_bg: parse_hex_color(&self.menu_bg)?,
            tooltip_bg: parse_hex_color(&self.tooltip_bg)?,
            tag_bg: parse_hex_color(&self.tag_bg)?,
            tag_active_bg: parse_hex_color(&self.tag_active_bg)?,
            input_bg: parse_hex_color(&self.input_bg)?,
            todo_color: parse_hex_color(&self.todo_color)?,
            in_progress_color: parse_hex_color(&self.in_progress_color)?,
            blocked_color: parse_hex_color(&self.blocked_color)?,
            completed_color: parse_hex_color(&self.completed_color)?,
            archived_color: parse_hex_color(&self.archived_color)?,
            importance_high_stripe: parse_hex_color(&self.importance_high_stripe)?,
            importance_low_stripe: parse_hex_color(&self.importance_low_stripe)?,
            urgency_high_stripe: parse_hex_color(&self.urgency_high_stripe)?,
            urgency_low_stripe: parse_hex_color(&self.urgency_low_stripe)?,
        })
    }
}

pub fn initialize_theme_files() -> Result<(), String> {
    fs::create_dir_all(THEMES_DIR)
        .map_err(|error| format!("Failed to create themes dir: {error}"))?;

    ensure_theme_file("dark.toml", DARK_THEME_TOML)?;
    ensure_theme_file("light.toml", LIGHT_THEME_TOML)?;

    Ok(())
}

fn ensure_theme_file(name: &str, contents: &str) -> Result<(), String> {
    let path = Path::new(THEMES_DIR).join(name);
    if path.exists() {
        return Ok(());
    }

    fs::write(&path, contents)
        .map_err(|error| format!("Failed to write {}: {error}", path.display()))
}

pub fn load_gui_settings() -> GuiSettings {
    fs::read_to_string(GUI_SETTINGS_PATH)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default()
}

pub fn save_gui_settings(settings: &GuiSettings) -> Result<(), String> {
    let content = toml::to_string_pretty(settings)
        .map_err(|error| format!("Failed to serialize settings: {error}"))?;
    fs::write(GUI_SETTINGS_PATH, content)
        .map_err(|error| format!("Failed to save settings: {error}"))
}

pub fn available_theme_names() -> Result<Vec<String>, String> {
    initialize_theme_files()?;

    let mut names = Vec::new();
    let entries =
        fs::read_dir(THEMES_DIR).map_err(|error| format!("Failed to read themes dir: {error}"))?;

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

    let path = theme_path(theme_name);
    load_theme_palette_from_path(&path, theme_name)
}

pub fn load_theme_palette_from_path(
    path: &Path,
    fallback_name: &str,
) -> Result<ThemePalette, String> {
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read theme '{}': {error}", path.display()))?;
    let file: ThemePaletteFile = toml::from_str(&content)
        .map_err(|error| format!("Failed to parse theme '{}': {error}", path.display()))?;

    file.into_palette(fallback_name)
}

pub fn import_theme_file(path: &Path) -> Result<String, String> {
    initialize_theme_files()?;

    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Invalid theme filename: {}", path.display()))?;
    let target = theme_path(stem);

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
    initialize_theme_files()?;

    let settings = load_gui_settings();
    let palette = load_theme_palette(&settings.selected_theme)
        .or_else(|_| load_theme_palette(DEFAULT_THEME_NAME))?;
    apply_theme_palette(palette.clone());
    Ok(palette)
}

pub fn theme_path(theme_name: &str) -> PathBuf {
    Path::new(THEMES_DIR).join(format!("{theme_name}.toml"))
}

fn parse_hex_color(input: &str) -> Result<Color, String> {
    let trimmed = input.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);

    if hex.len() != 6 {
        return Err(format!("Invalid color '{input}': expected 6 hex digits"));
    }

    let red = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| format!("Invalid red channel in color '{input}'"))?;
    let green = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| format!("Invalid green channel in color '{input}'"))?;
    let blue = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| format!("Invalid blue channel in color '{input}'"))?;

    Ok(Color::from_rgb8(red, green, blue))
}
