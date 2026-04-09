use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use iced::widget::{text_editor, Container, Row};
use iced::{Alignment, Element, Length, Sandbox, Settings, Theme};
use rfd::FileDialog;

use crate::files::{load_taskbar, save_taskbar, DEFAULT_TASKBAR_PATH};
use crate::tasks::{
    ImportanceFilter, PinnedFilter, StateFilter, TaskDraft, TaskImportance, TaskManager, TaskState,
    TaskUrgency, UrgencyFilter,
};

use super::settings::{
    apply_saved_theme, available_theme_names, load_gui_settings, load_theme_palette,
    save_gui_settings, GuiSettings,
};
use super::theme::{container_primary_style, task_state_button_label};

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
    ToggleShowDetailsAside(bool),
    SaveSettings,
    CloseSettingsMenu,
    SaveTaskFileAs,
    LoadTaskFileFrom,
    RequestClearAllTasks,
    ConfirmClearAllTasks,
    HoverTaskEnter(u32),
    HoverTaskExit(u32),
    ToggleStateMenu(u32),
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
    DetailTextAction(DetailField, text_editor::Action),
    SaveDetail,
    CloseDetail,
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

#[derive(Debug, Clone, Copy)]
pub enum DetailField {
    Name,
    Description,
    Tags,
    Dates,
}

pub struct Gui {
    pub manager: TaskManager,
    pub collapsed: HashSet<u32>,
    pub side_panel: Option<SidePanel>,
    pub show_settings_menu: bool,
    pub show_filter_menu: bool,
    pub active_theme_name: String,
    pub show_details_aside: bool,
    pub draft_theme_name: String,
    pub draft_show_details_aside: bool,
    pub available_theme_names: Vec<String>,
    pub settings_status: Option<String>,
    pub settings_confirm_clear_all: bool,
    pub draft_filter_tags: Vec<String>,
    pub draft_importance_filter: ImportanceFilter,
    pub draft_urgency_filter: UrgencyFilter,
    pub draft_state_filter: StateFilter,
    pub draft_pinned_filter: PinnedFilter,
    pub task_file_path: PathBuf,
    pub hovered_task: Option<u32>,
    pub state_menu_for: Option<u32>,
    pub detail_name: text_editor::Content,
    pub detail_description: text_editor::Content,
    pub detail_tags: text_editor::Content,
    pub detail_dates: text_editor::Content,
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

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        let _ = super::settings::initialize_theme_files();
        let theme_settings = load_gui_settings();
        let available_theme_names =
            available_theme_names().unwrap_or_else(|_| vec!["dark".to_string()]);
        let mut active_theme_name = theme_settings.selected_theme.clone();
        if !available_theme_names
            .iter()
            .any(|theme_name| theme_name == &active_theme_name)
        {
            active_theme_name = "dark".to_string();
        }
        let _ = apply_saved_theme().or_else(|_| {
            load_theme_palette(&active_theme_name).map(|palette| {
                super::theme::apply_theme_palette(palette.clone());
                palette
            })
        });

        let manager = match load_taskbar(DEFAULT_TASKBAR_PATH) {
            Ok(m) => m,
            Err(_) => TaskManager::new(),
        };
        let draft_available_tags = manager.available_tags.clone();
        let draft_filter_tags = manager.active_filter_tags.clone();
        let draft_importance_filter = manager.active_importance_filter.clone();
        let draft_urgency_filter = manager.active_urgency_filter.clone();
        let draft_state_filter = manager.active_state_filter.clone();
        let draft_pinned_filter = manager.active_pinned_filter.clone();

        Gui {
            manager,
            collapsed: HashSet::new(),
            side_panel: None,
            show_settings_menu: false,
            show_filter_menu: false,
            active_theme_name: active_theme_name.clone(),
            show_details_aside: theme_settings.show_details_aside,
            draft_theme_name: active_theme_name,
            draft_show_details_aside: theme_settings.show_details_aside,
            available_theme_names,
            settings_status: None,
            settings_confirm_clear_all: false,
            draft_filter_tags,
            draft_importance_filter,
            draft_urgency_filter,
            draft_state_filter,
            draft_pinned_filter,
            task_file_path: PathBuf::from(DEFAULT_TASKBAR_PATH),
            hovered_task: None,
            state_menu_for: None,
            detail_name: text_editor::Content::new(),
            detail_description: text_editor::Content::new(),
            detail_tags: text_editor::Content::new(),
            detail_dates: text_editor::Content::new(),
            draft_state: TaskState::Todo,
            draft_pinned: false,
            draft_tags: Vec::new(),
            draft_available_tags,
            show_tag_editor: false,
            tag_input_value: String::new(),
            draft_urgency: None,
            draft_importance: None,
            draft_due_date: None,
            draft_completed_at: None,
            active_date_panel: None,
            date_input_value: String::new(),
            date_selected_year: None,
            date_selected_month: None,
            date_selected_day: None,
            date_selected_hour: None,
            date_selected_minute: None,
            delete_confirmation_for: None,
        }
    }

    fn title(&self) -> String {
        "Task Manager".into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleCollapse(id) => {
                if self.collapsed.contains(&id) {
                    self.collapsed.remove(&id);
                } else {
                    self.collapsed.insert(id);
                }
                self.state_menu_for = None;
            }
            Message::ToggleDetail(id) => {
                if self.side_panel == Some(SidePanel::Detail(id)) {
                    self.side_panel = None;
                } else {
                    self.side_panel = Some(SidePanel::Detail(id));
                }
                self.state_menu_for = None;
                self.sync_detail_content();
            }
            Message::OpenCreateRoot => {
                self.side_panel = Some(SidePanel::Create(None));
                self.state_menu_for = None;
                self.sync_detail_content();
            }
            Message::OpenCreateChild(id) => {
                self.side_panel = Some(SidePanel::Create(Some(id)));
                self.state_menu_for = None;
                self.sync_detail_content();
            }
            Message::ToggleSettingsMenu => {
                self.show_settings_menu = !self.show_settings_menu;
                self.show_filter_menu = false;
                if self.show_settings_menu {
                    self.available_theme_names = available_theme_names()
                        .unwrap_or_else(|_| vec![self.active_theme_name.clone()]);
                    self.draft_theme_name = self.active_theme_name.clone();
                    self.draft_show_details_aside = self.show_details_aside;
                    self.settings_status = None;
                    self.settings_confirm_clear_all = false;
                }
            }
            Message::ToggleFilterMenu => {
                self.show_filter_menu = !self.show_filter_menu;
                self.show_settings_menu = false;
                self.draft_filter_tags = self.manager.active_filter_tags.clone();
                self.draft_importance_filter = self.manager.active_importance_filter.clone();
                self.draft_urgency_filter = self.manager.active_urgency_filter.clone();
                self.draft_state_filter = self.manager.active_state_filter.clone();
                self.draft_pinned_filter = self.manager.active_pinned_filter.clone();
            }
            Message::ToggleDraftFilterTag(tag) => {
                if self
                    .draft_filter_tags
                    .iter()
                    .any(|selected| selected == &tag)
                {
                    self.draft_filter_tags.retain(|selected| selected != &tag);
                } else if self
                    .manager
                    .available_tags
                    .iter()
                    .any(|known| known == &tag)
                {
                    self.draft_filter_tags.push(tag);
                    self.draft_filter_tags.sort();
                    self.draft_filter_tags.dedup();
                }
            }
            Message::ClearDraftFilterTags => {
                self.draft_filter_tags.clear();
                self.draft_importance_filter = ImportanceFilter::Any;
                self.draft_urgency_filter = UrgencyFilter::Any;
                self.draft_state_filter = StateFilter::Any;
                self.draft_pinned_filter = PinnedFilter::Any;
            }
            Message::SearchQueryChanged(value) => {
                self.manager.set_active_search_query(value);
            }
            Message::ClearSearchQuery => {
                self.manager.clear_active_search_query();
            }
            Message::SelectDraftImportanceFilter(filter) => {
                if self.draft_importance_filter == filter {
                    self.draft_importance_filter = ImportanceFilter::Any;
                } else {
                    self.draft_importance_filter = filter;
                }
            }
            Message::SelectDraftUrgencyFilter(filter) => {
                if self.draft_urgency_filter == filter {
                    self.draft_urgency_filter = UrgencyFilter::Any;
                } else {
                    self.draft_urgency_filter = filter;
                }
            }
            Message::SelectDraftStateFilter(filter) => {
                if self.draft_state_filter == filter {
                    self.draft_state_filter = StateFilter::Any;
                } else {
                    self.draft_state_filter = filter;
                }
            }
            Message::SelectDraftPinnedFilter(filter) => {
                if self.draft_pinned_filter == filter {
                    self.draft_pinned_filter = PinnedFilter::Any;
                } else {
                    self.draft_pinned_filter = filter;
                }
            }
            Message::ApplyFilterSelection => {
                self.manager
                    .set_active_filter_tags(self.draft_filter_tags.clone());
                self.manager
                    .set_active_importance_filter(self.draft_importance_filter.clone());
                self.manager
                    .set_active_urgency_filter(self.draft_urgency_filter.clone());
                self.manager
                    .set_active_state_filter(self.draft_state_filter.clone());
                self.manager
                    .set_active_pinned_filter(self.draft_pinned_filter.clone());
                self.show_filter_menu = false;
            }
            Message::CancelFilterSelection => {
                self.show_filter_menu = false;
                self.draft_filter_tags = self.manager.active_filter_tags.clone();
                self.draft_importance_filter = self.manager.active_importance_filter.clone();
                self.draft_urgency_filter = self.manager.active_urgency_filter.clone();
                self.draft_state_filter = self.manager.active_state_filter.clone();
                self.draft_pinned_filter = self.manager.active_pinned_filter.clone();
            }
            Message::SelectTheme(theme_name) => {
                self.draft_theme_name = theme_name;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
            }
            Message::ToggleShowDetailsAside(value) => {
                self.draft_show_details_aside = value;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
            }
            Message::SaveSettings => match load_theme_palette(&self.draft_theme_name) {
                Ok(palette) => {
                    super::theme::apply_theme_palette(palette);
                    self.active_theme_name = self.draft_theme_name.clone();
                    self.show_details_aside = self.draft_show_details_aside;
                    let settings = GuiSettings {
                        selected_theme: self.active_theme_name.clone(),
                        show_details_aside: self.show_details_aside,
                    };
                    match save_gui_settings(&settings) {
                        Ok(()) => {
                            self.available_theme_names = available_theme_names()
                                .unwrap_or_else(|_| vec![self.active_theme_name.clone()]);
                            self.settings_status = Some("Settings saved.".to_string());
                            self.show_settings_menu = false;
                            self.settings_confirm_clear_all = false;
                        }
                        Err(error) => {
                            self.settings_status = Some(error);
                        }
                    }
                }
                Err(error) => {
                    self.settings_status = Some(error);
                }
            },
            Message::CloseSettingsMenu => {
                self.show_settings_menu = false;
                self.draft_theme_name = self.active_theme_name.clone();
                self.draft_show_details_aside = self.show_details_aside;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
            }
            Message::SaveTaskFileAs => {
                self.save_task_file_as();
            }
            Message::LoadTaskFileFrom => {
                self.load_task_file_from();
            }
            Message::RequestClearAllTasks => {
                self.settings_confirm_clear_all = true;
                self.settings_status =
                    Some("Click 'Confirm Clear' to remove all tasks.".to_string());
            }
            Message::ConfirmClearAllTasks => {
                self.manager = TaskManager::new();
                self.collapsed.clear();
                self.side_panel = None;
                self.state_menu_for = None;
                self.hovered_task = None;
                self.delete_confirmation_for = None;
                self.settings_confirm_clear_all = false;
                self.persist_changes();
                self.sync_detail_content();
                self.settings_status = Some(format!(
                    "Cleared all tasks and saved {}.",
                    self.task_file_path.display()
                ));
            }
            Message::HoverTaskEnter(id) => {
                self.hovered_task = Some(id);
            }
            Message::HoverTaskExit(id) => {
                if self.hovered_task == Some(id) {
                    self.hovered_task = None;
                }
            }
            Message::ToggleStateMenu(id) => {
                if self.state_menu_for == Some(id) {
                    self.state_menu_for = None;
                } else {
                    self.state_menu_for = Some(id);
                }
            }
            Message::TogglePinned(id) => {
                if self.side_panel == Some(SidePanel::Detail(id)) {
                    self.draft_pinned = !self.draft_pinned;
                } else {
                    let _ = self.manager.toggle_task_pinned(id);
                    self.persist_changes();
                    self.sync_detail_content();
                }
                self.delete_confirmation_for = None;
            }
            Message::SelectState(id, new_state) => {
                if self.side_panel == Some(SidePanel::Detail(id)) {
                    self.draft_state = new_state;
                } else {
                    let _ = self.manager.set_task_state(id, new_state);
                    self.persist_changes();
                    self.sync_detail_content();
                }
                self.state_menu_for = None;
                self.delete_confirmation_for = None;
            }
            Message::UndoLastChange => {
                if self.manager.undo_last_change().is_ok() {
                    self.persist_changes();
                    self.state_menu_for = None;
                    self.delete_confirmation_for = None;
                    self.sync_detail_content();
                }
            }
            Message::RequestDelete(id) => {
                if self
                    .manager
                    .root
                    .search_by_id_ref(id)
                    .map(|task| task.subtasks.is_empty())
                    .unwrap_or(false)
                {
                    self.delete_task(id);
                } else {
                    self.delete_confirmation_for = Some(id);
                }
            }
            Message::ConfirmDelete(id) => {
                self.delete_task(id);
            }
            Message::CancelDelete => {
                self.delete_confirmation_for = None;
            }
            Message::CycleUrgency => {
                self.draft_urgency = match self.draft_urgency {
                    None => Some(TaskUrgency::Low),
                    Some(TaskUrgency::Low) => Some(TaskUrgency::High),
                    Some(TaskUrgency::High) => None,
                };
                self.delete_confirmation_for = None;
            }
            Message::CycleImportance => {
                self.draft_importance = match self.draft_importance {
                    None => Some(TaskImportance::Low),
                    Some(TaskImportance::Low) => Some(TaskImportance::High),
                    Some(TaskImportance::High) => None,
                };
                self.delete_confirmation_for = None;
            }
            Message::ToggleTaskTag(tag) => {
                if self.draft_tags.iter().any(|existing| existing == &tag) {
                    self.draft_tags.retain(|existing| existing != &tag);
                } else if self
                    .draft_available_tags
                    .iter()
                    .any(|existing| existing == &tag)
                {
                    self.draft_tags.push(tag);
                    self.draft_tags.sort();
                    self.draft_tags.dedup();
                }
                self.delete_confirmation_for = None;
            }
            Message::ToggleTagEditor => {
                self.show_tag_editor = !self.show_tag_editor;
                if !self.show_tag_editor {
                    self.tag_input_value.clear();
                }
                self.delete_confirmation_for = None;
            }
            Message::TagInputChanged(value) => {
                self.tag_input_value = value;
                self.delete_confirmation_for = None;
            }
            Message::AddDraftTag => {
                let tag = self.tag_input_value.trim();
                if !tag.is_empty() && self.draft_available_tags.len() < 3 {
                    if !self
                        .draft_available_tags
                        .iter()
                        .any(|existing| existing == tag)
                    {
                        self.draft_available_tags.push(tag.to_string());
                    }
                    if !self.draft_tags.iter().any(|existing| existing == tag) {
                        self.draft_tags.push(tag.to_string());
                    }
                    self.draft_available_tags.sort();
                    self.draft_available_tags.dedup();
                    self.draft_tags.sort();
                    self.draft_tags.dedup();
                    self.tag_input_value.clear();
                }
                self.delete_confirmation_for = None;
            }
            Message::ToggleDatePanel(field) => {
                if self.active_date_panel == Some(field) {
                    self.active_date_panel = None;
                    self.date_input_value.clear();
                    self.clear_date_picker_state();
                } else {
                    self.active_date_panel = Some(field);
                    self.date_input_value = self
                        .date_value(field)
                        .map(super::theme::format_date)
                        .unwrap_or_default();
                    self.sync_date_picker_state(self.date_value(field).unwrap_or_else(Utc::now));
                }
                self.delete_confirmation_for = None;
            }
            Message::DateInputChanged(value) => {
                self.date_input_value = value;
                if let Ok(parsed) =
                    NaiveDateTime::parse_from_str(&self.date_input_value, "%Y-%m-%d %H:%M")
                {
                    self.sync_date_picker_state(DateTime::<Utc>::from_naive_utc_and_offset(
                        parsed, Utc,
                    ));
                }
                self.delete_confirmation_for = None;
            }
            Message::SelectDateYear(year) => {
                self.date_selected_year = Some(year);
                self.clamp_selected_day();
                self.sync_text_from_picker_state();
            }
            Message::SelectDateMonth(month) => {
                self.date_selected_month = Some(month);
                self.clamp_selected_day();
                self.sync_text_from_picker_state();
            }
            Message::SelectDateDay(day) => {
                self.date_selected_day = Some(day);
                self.sync_text_from_picker_state();
            }
            Message::SelectDateHour(hour) => {
                self.date_selected_hour = Some(hour);
                self.sync_text_from_picker_state();
            }
            Message::SelectDateMinute(minute) => {
                self.date_selected_minute = Some(minute);
                self.sync_text_from_picker_state();
            }
            Message::ApplyDateSelection => {
                if let Some(field) = self.active_date_panel {
                    if let Ok(parsed) =
                        NaiveDateTime::parse_from_str(&self.date_input_value, "%Y-%m-%d %H:%M")
                    {
                        let value = DateTime::<Utc>::from_naive_utc_and_offset(parsed, Utc);
                        match field {
                            DateField::DueDate => self.draft_due_date = Some(value),
                            DateField::CompletedAt => self.draft_completed_at = Some(value),
                        }
                        self.sync_dates_content();
                        self.active_date_panel = None;
                        self.clear_date_picker_state();
                    }
                }
                self.delete_confirmation_for = None;
            }
            Message::ClearDateSelection => {
                if let Some(field) = self.active_date_panel {
                    match field {
                        DateField::DueDate => self.draft_due_date = None,
                        DateField::CompletedAt => self.draft_completed_at = None,
                    }
                    self.sync_dates_content();
                    self.active_date_panel = None;
                    self.date_input_value.clear();
                    self.clear_date_picker_state();
                }
                self.delete_confirmation_for = None;
            }
            Message::DetailNameChanged(value) => {
                self.detail_name = text_editor::Content::with_text(&value);
                self.delete_confirmation_for = None;
            }
            Message::DetailTextAction(field, action) => {
                if matches!(field, DetailField::Name | DetailField::Description)
                    || !action.is_edit()
                {
                    self.detail_content_mut(field).perform(action);
                }
                self.delete_confirmation_for = None;
            }
            Message::CloseDetail => {
                self.side_panel = None;
                self.sync_detail_content();
            }
            Message::SaveDetail => {
                self.save_detail();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if self.show_settings_menu {
            return self.view_settings_overlay();
        }
        if self.show_filter_menu {
            return self.view_filter_overlay();
        }

        let mut main_row = Row::new()
            .spacing(16)
            .align_items(Alignment::Start)
            .push(self.view_task_list());

        if self.show_details_aside {
            if let Some(panel) = self.side_panel {
                match panel {
                    SidePanel::Detail(task_id) => {
                        if let Some(task) = self.manager.root.search_by_id_ref(task_id) {
                            main_row = main_row.push(self.view_detail(task));
                        }
                    }
                    SidePanel::Create(parent_id) => {
                        main_row = main_row.push(self.view_create_task(parent_id));
                    }
                }
            }
        }

        let content = Container::new(
            Container::new(main_row.padding(16))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_primary_style);

        if !self.show_details_aside {
            if let Some(overlay) = self.view_side_panel_overlay() {
                return overlay;
            }
        }

        content.into()
    }
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskState::Todo => "Todo",
            TaskState::InProgress => "In Progress",
            TaskState::Blocked => "Blocked",
            TaskState::Completed => "Completed",
            TaskState::Archived => "Archived",
        };
        write!(f, "{}", s)
    }
}

pub fn run_gui_app() -> Result<(), String> {
    Gui::run(Settings {
        window: iced::window::Settings::default(),
        ..Settings::default()
    })
    .map_err(|e| e.to_string())
}

impl Gui {
    fn persist_changes(&self) {
        if let Err(error) = save_taskbar(&self.task_file_path, &self.manager) {
            eprintln!("Failed to save taskbar: {}", error);
        }
    }

    fn save_task_file_as(&mut self) {
        let dialog = FileDialog::new()
            .add_filter("Task files", &["json"])
            .set_file_name(
                self.task_file_path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or(DEFAULT_TASKBAR_PATH),
            );

        let Some(path) = dialog.save_file() else {
            self.settings_status = Some("Save cancelled.".to_string());
            self.settings_confirm_clear_all = false;
            return;
        };

        match save_taskbar(&path, &self.manager) {
            Ok(()) => {
                self.task_file_path = path.clone();
                self.settings_status = Some(format!("Saved tasks to {}.", path.display()));
                self.settings_confirm_clear_all = false;
            }
            Err(error) => {
                self.settings_status = Some(error);
            }
        }
    }

    fn load_task_file_from(&mut self) {
        let dialog = FileDialog::new().add_filter("Task files", &["json"]);

        let Some(path) = dialog.pick_file() else {
            self.settings_status = Some("Load cancelled.".to_string());
            self.settings_confirm_clear_all = false;
            return;
        };

        match load_taskbar(&path) {
            Ok(manager) => {
                self.manager = manager;
                self.task_file_path = path.clone();
                self.collapsed.clear();
                self.side_panel = None;
                self.state_menu_for = None;
                self.hovered_task = None;
                self.delete_confirmation_for = None;
                self.settings_status = Some(format!("Loaded tasks from {}.", path.display()));
                self.settings_confirm_clear_all = false;
                self.sync_detail_content();
            }
            Err(error) => {
                self.settings_status = Some(error);
            }
        }
    }

    fn detail_content_mut(&mut self, field: DetailField) -> &mut text_editor::Content {
        match field {
            DetailField::Name => &mut self.detail_name,
            DetailField::Description => &mut self.detail_description,
            DetailField::Tags => &mut self.detail_tags,
            DetailField::Dates => &mut self.detail_dates,
        }
    }

    fn sync_detail_content(&mut self) {
        let Some(panel) = self.side_panel else {
            self.reset_detail_draft();
            return;
        };

        if let SidePanel::Create(_) = panel {
            self.reset_detail_draft();
            self.sync_dates_content();
            return;
        }

        let SidePanel::Detail(task_id) = panel else {
            return;
        };

        let Some(task) = self.manager.root.search_by_id_ref(task_id) else {
            self.side_panel = None;
            self.reset_detail_draft();
            return;
        };

        self.detail_name = text_editor::Content::with_text(&task.name);
        self.detail_description = text_editor::Content::with_text(&task.description);
        self.draft_state = task.state.clone();
        self.draft_pinned = task.pinned;
        self.draft_tags = task.tags.clone();
        self.draft_available_tags = self.manager.available_tags.clone();
        for tag in &task.tags {
            if !self
                .draft_available_tags
                .iter()
                .any(|existing| existing == tag)
                && self.draft_available_tags.len() < 3
            {
                self.draft_available_tags.push(tag.clone());
            }
        }
        self.show_tag_editor = false;
        self.tag_input_value.clear();
        self.draft_urgency = task.urgency.clone();
        self.draft_importance = task.importance.clone();
        self.draft_due_date = task.times.due_date;
        self.draft_completed_at = task.times.completed_at;
        let tags_text = if task.tags.is_empty() {
            String::new()
        } else {
            format!("Tags\n{}", task.tags.join("\n"))
        };
        self.active_date_panel = None;
        self.date_input_value.clear();
        self.clear_date_picker_state();
        self.delete_confirmation_for = None;
        self.detail_tags = text_editor::Content::with_text(&tags_text);
        self.sync_dates_content();
    }

    fn sync_dates_content(&mut self) {
        let mut lines = Vec::new();

        if let Some(panel) = self.side_panel {
            if let SidePanel::Detail(task_id) = panel {
                if let Some(task) = self.manager.root.search_by_id_ref(task_id) {
                    lines.push(format!(
                        "Created: {}",
                        super::theme::format_date(task.times.created_at)
                    ));
                    lines.push(format!(
                        "Updated: {}",
                        super::theme::format_date(task.times.updated_at)
                    ));
                }
            }
        }

        self.detail_dates = text_editor::Content::with_text(&lines.join("\n"));
    }

    fn date_value(&self, field: DateField) -> Option<DateTime<Utc>> {
        match field {
            DateField::DueDate => self.draft_due_date,
            DateField::CompletedAt => self.draft_completed_at,
        }
    }

    pub fn urgency_label(&self) -> String {
        self.draft_urgency
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "Urgency: None".to_string())
    }

    pub fn importance_label(&self) -> String {
        self.draft_importance
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "Importance: None".to_string())
    }

    pub fn date_button_label(&self, field: DateField) -> String {
        match (field, self.date_value(field)) {
            (DateField::DueDate, Some(value)) => {
                format!("Due: {}", super::theme::format_date(value))
            }
            (DateField::DueDate, None) => "Due: None".to_string(),
            (DateField::CompletedAt, Some(value)) => {
                format!("Completed: {}", super::theme::format_date(value))
            }
            (DateField::CompletedAt, None) => "Completed: None".to_string(),
        }
    }

    fn editor_text(content: &text_editor::Content) -> String {
        content.text().trim().to_string()
    }

    fn reset_detail_draft(&mut self) {
        self.detail_name = text_editor::Content::new();
        self.detail_description = text_editor::Content::new();
        self.detail_tags = text_editor::Content::new();
        self.detail_dates = text_editor::Content::new();
        self.draft_state = TaskState::Todo;
        self.draft_pinned = false;
        self.draft_tags.clear();
        self.draft_available_tags = self.manager.available_tags.clone();
        self.show_tag_editor = false;
        self.tag_input_value.clear();
        self.draft_urgency = None;
        self.draft_importance = None;
        self.draft_due_date = None;
        self.draft_completed_at = None;
        self.active_date_panel = None;
        self.date_input_value.clear();
        self.clear_date_picker_state();
        self.delete_confirmation_for = None;
    }

    fn clear_date_picker_state(&mut self) {
        self.date_selected_year = None;
        self.date_selected_month = None;
        self.date_selected_day = None;
        self.date_selected_hour = None;
        self.date_selected_minute = None;
    }

    fn sync_date_picker_state(&mut self, value: DateTime<Utc>) {
        self.date_selected_year = Some(value.year());
        self.date_selected_month = Some(value.month());
        self.date_selected_day = Some(value.day());
        self.date_selected_hour = Some(value.hour());
        self.date_selected_minute = Some(value.minute());
    }

    fn sync_text_from_picker_state(&mut self) {
        let (Some(year), Some(month), Some(day), Some(hour), Some(minute)) = (
            self.date_selected_year,
            self.date_selected_month,
            self.date_selected_day,
            self.date_selected_hour,
            self.date_selected_minute,
        ) else {
            return;
        };

        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            if let Some(datetime) = date.and_hms_opt(hour, minute, 0) {
                self.date_input_value = datetime.format("%Y-%m-%d %H:%M").to_string();
            }
        }
    }

    pub fn year_options(&self) -> Vec<i32> {
        let current_year = self
            .active_date_panel
            .and_then(|field| self.date_value(field))
            .map(|value| value.year())
            .unwrap_or_else(|| Utc::now().year());

        ((current_year - 5)..=(current_year + 5)).collect()
    }

    pub fn state_button_label(&self) -> String {
        task_state_button_label(&self.draft_state)
    }

    pub fn month_options(&self) -> Vec<u32> {
        (1..=12).collect()
    }

    pub fn day_options(&self) -> Vec<u32> {
        let year = self.date_selected_year.unwrap_or_else(|| Utc::now().year());
        let month = self.date_selected_month.unwrap_or(1);
        let mut days = Vec::new();

        for day in 1..=31 {
            if NaiveDate::from_ymd_opt(year, month, day).is_some() {
                days.push(day);
            }
        }

        days
    }

    pub fn hour_options(&self) -> Vec<u32> {
        (0..24).collect()
    }

    pub fn minute_options(&self) -> Vec<u32> {
        (0..60).collect()
    }

    pub fn can_undo(&self) -> bool {
        self.manager.can_undo()
    }

    pub fn can_add_more_tags(&self) -> bool {
        self.draft_available_tags.len() < 3
    }

    pub fn task_has_tag(&self, tag: &str) -> bool {
        self.draft_tags.iter().any(|existing| existing == tag)
    }

    fn clamp_selected_day(&mut self) {
        if let Some(day) = self.date_selected_day {
            if !self.day_options().contains(&day) {
                self.date_selected_day = self.day_options().into_iter().last();
            }
        }
    }

    fn draft_from_form(&self) -> TaskDraft {
        TaskDraft {
            name: Self::editor_text(&self.detail_name),
            description: Self::editor_text(&self.detail_description),
            state: self.draft_state.clone(),
            urgency: self.draft_urgency.clone(),
            importance: self.draft_importance.clone(),
            tags: self.draft_tags.clone(),
            pinned: self.draft_pinned,
            due_date: self.draft_due_date,
            completed_at: self.draft_completed_at,
        }
    }

    fn save_detail(&mut self) {
        let Some(panel) = self.side_panel else {
            return;
        };

        let draft = self.draft_from_form();
        if draft.name.is_empty() {
            return;
        }

        let result = match panel {
            SidePanel::Detail(task_id) => self
                .manager
                .save_task_detail(Some(task_id), 0, draft, self.draft_available_tags.clone())
                .map(|_| ()),
            SidePanel::Create(parent_id) => self
                .manager
                .save_task_detail(
                    None,
                    parent_id.unwrap_or(0),
                    draft,
                    self.draft_available_tags.clone(),
                )
                .map(|_| ()),
        };

        if result.is_ok() {
            self.persist_changes();
            self.state_menu_for = None;
            self.delete_confirmation_for = None;
            match panel {
                SidePanel::Detail(task_id) => {
                    self.side_panel = Some(SidePanel::Detail(task_id));
                }
                SidePanel::Create(_) => {
                    self.side_panel = None;
                }
            }
            self.sync_detail_content();
        }
    }

    fn delete_task(&mut self, id: u32) {
        if self.manager.delete_task(id).is_ok() {
            self.collapsed.remove(&id);
            self.state_menu_for = None;
            self.delete_confirmation_for = None;
            if self.side_panel == Some(SidePanel::Detail(id)) {
                self.side_panel = None;
            }
            self.persist_changes();
            self.sync_detail_content();
        }
    }
}
