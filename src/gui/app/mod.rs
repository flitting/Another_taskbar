mod detail_state;
mod update;

use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use iced::keyboard::key::Named;
use iced::widget::text_editor;
use iced::{event, executor, keyboard, Application, Command, Font, Settings, Subscription, Theme};

use crate::tasks::{
    ImportanceFilter, PinnedFilter, StateFilter, TaskImportance, TaskManager, TaskState,
    TaskUrgency, UrgencyFilter,
};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleCollapse(u32),
    ToggleDetail(u32),
    OpenCreateRoot,
    OpenCreateChild(u32),
    ToggleSettingsMenu,
    ToggleFilterMenu,
    ToggleDraftFilterTag(String),
    ClearDraftFilterTags,
    SearchQueryChanged(String),
    ClearSearchQuery,
    SelectDraftImportanceFilter(ImportanceFilter),
    SelectDraftUrgencyFilter(UrgencyFilter),
    SelectDraftStateFilter(StateFilter),
    SelectDraftPinnedFilter(PinnedFilter),
    ApplyFilterSelection,
    CancelFilterSelection,
    SelectTheme(String),
    SelectFont(String),
    SelectSymbolFont(String),
    ToggleShowDetailsAside(bool),
    SaveSettings,
    CloseSettingsMenu,
    SaveTaskFileAs,
    LoadTaskFileFrom,
    RequestClearAllTasks,
    ConfirmClearAllTasks,
    RequestClearAllDataAndExit,
    ConfirmClearAllDataAndExit,
    HoverTaskEnter(u32),
    HoverTaskExit(u32),
    TogglePinned(u32),
    SelectState(u32, TaskState),
    UndoLastChange,
    RequestDelete(u32),
    ConfirmDelete(u32),
    CancelDelete,
    CycleUrgency,
    CycleImportance,
    ToggleTaskTag(String),
    ToggleTagEditor,
    TagInputChanged(String),
    AddDraftTag,
    ToggleDatePanel(DateField),
    DateInputChanged(String),
    SelectDateYear(i32),
    SelectDateMonth(u32),
    SelectDateDay(u32),
    SelectDateHour(u32),
    SelectDateMinute(u32),
    ApplyDateSelection,
    ClearDateSelection,
    DetailNameChanged(String),
    DetailDescriptionAction(text_editor::Action),
    SelectAllDetailDescription,
    SaveDetail,
    CloseDetail,
    TriggerEscapeShortcut,
    TriggerSubmitShortcut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidePanel {
    Detail(u32),
    Create(Option<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateField {
    DueDate,
    CompletedAt,
}

pub struct Gui {
    pub manager: TaskManager,
    pub collapsed: HashSet<u32>,
    pub side_panel: Option<SidePanel>,
    pub show_settings_menu: bool,
    pub show_filter_menu: bool,
    pub active_theme_name: String,
    pub active_font_name: String,
    pub active_font: Font,
    pub active_symbol_font_name: String,
    pub active_symbol_font: Font,
    pub show_details_aside: bool,
    pub draft_theme_name: String,
    pub draft_font_name: String,
    pub draft_symbol_font_name: String,
    pub draft_show_details_aside: bool,
    pub available_theme_names: Vec<String>,
    pub available_font_names: Vec<String>,
    pub available_symbol_font_names: Vec<String>,
    pub settings_status: Option<String>,
    pub settings_confirm_clear_all: bool,
    pub settings_confirm_clear_data_and_exit: bool,
    pub draft_filter_tags: Vec<String>,
    pub draft_importance_filter: ImportanceFilter,
    pub draft_urgency_filter: UrgencyFilter,
    pub draft_state_filter: StateFilter,
    pub draft_pinned_filter: PinnedFilter,
    pub task_file_path: PathBuf,
    pub hovered_task: Option<u32>,
    pub detail_name: text_editor::Content,
    pub detail_description: text_editor::Content,
    pub detail_description_focused: bool,
    pub draft_state: TaskState,
    pub draft_pinned: bool,
    pub draft_tags: Vec<String>,
    pub draft_available_tags: Vec<String>,
    pub show_tag_editor: bool,
    pub tag_input_value: String,
    pub draft_urgency: Option<TaskUrgency>,
    pub draft_importance: Option<TaskImportance>,
    pub draft_due_date: Option<DateTime<Utc>>,
    pub draft_completed_at: Option<DateTime<Utc>>,
    pub active_date_panel: Option<DateField>,
    pub date_input_value: String,
    pub date_selected_year: Option<i32>,
    pub date_selected_month: Option<u32>,
    pub date_selected_day: Option<u32>,
    pub date_selected_hour: Option<u32>,
    pub date_selected_minute: Option<u32>,
    pub delete_confirmation_for: Option<u32>,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::new_app(), Command::none())
    }

    fn title(&self) -> String {
        "Task Manager".into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        self.handle_message(message)
    }

    fn view(&self) -> iced::Element<'_, Message> {
        self.view_main()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, _status| match event {
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if modifiers.command()
                    && matches!(key.as_ref(), keyboard::Key::Character("a" | "A")) =>
            {
                Some(Message::SelectAllDetailDescription)
            }
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if !modifiers.command()
                    && !modifiers.alt()
                    && matches!(key.as_ref(), keyboard::Key::Named(Named::Escape)) =>
            {
                Some(Message::TriggerEscapeShortcut)
            }
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if !modifiers.command()
                    && !modifiers.alt()
                    && matches!(key.as_ref(), keyboard::Key::Named(Named::Enter)) =>
            {
                Some(Message::TriggerSubmitShortcut)
            }
            _ => None,
        })
    }
}

pub fn run_gui_app() -> Result<(), String> {
    crate::bootstrap::initialize_app_storage()?;

    let gui_settings = crate::gui::settings::load_gui_settings();
    let active_font_name = crate::gui::settings::normalize_font_name(&gui_settings.selected_font);
    let active_font = crate::gui::settings::font_option(&active_font_name)
        .map(|option| option.font)
        .unwrap_or_else(crate::gui::settings::default_font);
    let active_symbol_font_name =
        crate::gui::settings::normalize_symbol_font_name(&gui_settings.selected_symbol_font);
    let _active_symbol_font = crate::gui::settings::symbol_font_option(&active_symbol_font_name)
        .map(|option| option.font)
        .unwrap_or_else(crate::gui::settings::default_symbol_font);

    Gui::run(Settings {
        window: iced::window::Settings::default(),
        fonts: crate::gui::settings::bundled_font_bytes_with_fallback(),
        default_font: active_font,
        ..Settings::default()
    })
    .map_err(|error| error.to_string())
}
