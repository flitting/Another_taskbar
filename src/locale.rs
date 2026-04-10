use std::collections::HashMap;
use std::fmt;
use std::sync::{OnceLock, RwLock};

use serde::{Deserialize, Serialize};

const EN_TOML: &str = include_str!("../locales/en.toml");
const ZH_CN_TOML: &str = include_str!("../locales/zh-CN.toml");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppLanguage {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "zh-CN")]
    ChineseSimplified,
}

impl Default for AppLanguage {
    fn default() -> Self {
        Self::English
    }
}

impl fmt::Display for AppLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.native_name())
    }
}

impl AppLanguage {
    pub fn all() -> Vec<Self> {
        vec![Self::English, Self::ChineseSimplified]
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ChineseSimplified => "中文",
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::ChineseSimplified => "zh-CN",
        }
    }
}

#[derive(Debug, Deserialize)]
struct LocaleFile {
    strings: HashMap<String, String>,
}

static CURRENT_LANGUAGE: OnceLock<RwLock<AppLanguage>> = OnceLock::new();
static LOCALES: OnceLock<HashMap<AppLanguage, HashMap<String, String>>> = OnceLock::new();

fn language_store() -> &'static RwLock<AppLanguage> {
    CURRENT_LANGUAGE.get_or_init(|| RwLock::new(AppLanguage::default()))
}

fn locale_map() -> &'static HashMap<AppLanguage, HashMap<String, String>> {
    LOCALES.get_or_init(|| {
        let mut locales = HashMap::new();
        locales.insert(AppLanguage::English, parse_locale(EN_TOML));
        locales.insert(AppLanguage::ChineseSimplified, parse_locale(ZH_CN_TOML));
        locales
    })
}

fn parse_locale(contents: &str) -> HashMap<String, String> {
    toml::from_str::<LocaleFile>(contents)
        .map(|file| file.strings)
        .unwrap_or_default()
}

pub fn set_current_language(language: AppLanguage) {
    if let Ok(mut current) = language_store().write() {
        *current = language;
    }
}

pub fn current_language() -> AppLanguage {
    language_store()
        .read()
        .map(|language| *language)
        .unwrap_or_default()
}

pub fn text(key: &str) -> String {
    text_for(current_language(), key)
}

pub fn text_for(language: AppLanguage, key: &str) -> String {
    locale_map()
        .get(&language)
        .and_then(|strings| strings.get(key))
        .cloned()
        .or_else(|| {
            locale_map()
                .get(&AppLanguage::English)
                .and_then(|strings| strings.get(key))
                .cloned()
        })
        .unwrap_or_else(|| key.to_string())
}

pub fn text_with_args(key: &str, args: &[(&str, String)]) -> String {
    let mut value = text(key);
    for (name, replacement) in args {
        value = value.replace(&format!("{{{name}}}"), replacement);
    }
    value
}

pub fn task_state_label(state: &crate::tasks::TaskState) -> String {
    match state {
        crate::tasks::TaskState::Todo => text("state_todo"),
        crate::tasks::TaskState::InProgress => text("state_in_progress"),
        crate::tasks::TaskState::Blocked => text("state_blocked"),
        crate::tasks::TaskState::Completed => text("state_completed"),
        crate::tasks::TaskState::Archived => text("state_archived"),
    }
}

pub fn task_urgency_label(urgency: &crate::tasks::TaskUrgency) -> String {
    match urgency {
        crate::tasks::TaskUrgency::Low => text("urgency_low"),
        crate::tasks::TaskUrgency::High => text("urgency_high"),
    }
}

pub fn task_importance_label(importance: &crate::tasks::TaskImportance) -> String {
    match importance {
        crate::tasks::TaskImportance::Low => text("importance_low"),
        crate::tasks::TaskImportance::High => text("importance_high"),
    }
}
