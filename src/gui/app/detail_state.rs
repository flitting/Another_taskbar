use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use iced::widget::{text_editor, Container, Row};
use iced::{Alignment, Command, Element, Length};
use iced_aw::Modal;
use rfd::FileDialog;

use crate::files::{load_taskbar, save_taskbar, TaskbarDefaultPath, DEFAULT_TASKBAR_FILE_NAME};
use crate::gui::settings::{
    apply_saved_theme, available_font_names, available_symbol_font_names, available_theme_names,
    font_option, load_gui_settings, load_theme_palette, normalize_font_name,
    normalize_symbol_font_name, save_gui_settings, symbol_font_option, GuiSettings,
};
use crate::gui::theme::container_primary_style;
use crate::tasks::{TaskDraft, TaskState};

use super::{DateField, Gui, Message, SidePanel};

impl Gui {
    pub(super) fn new_app() -> Self {
        let _ = crate::bootstrap::initialize_app_storage();
        let default_taskbar_path = TaskbarDefaultPath::resolve()
            .unwrap_or_else(|_| std::path::PathBuf::from(DEFAULT_TASKBAR_FILE_NAME));
        let theme_settings = load_gui_settings();
        let active_font_name = normalize_font_name(&theme_settings.selected_font);
        let active_font = font_option(&active_font_name)
            .map(|option| option.font)
            .unwrap_or_else(crate::gui::settings::default_font);
        let active_symbol_font_name =
            normalize_symbol_font_name(&theme_settings.selected_symbol_font);
        let active_symbol_font = symbol_font_option(&active_symbol_font_name)
            .map(|option| option.font)
            .unwrap_or_else(crate::gui::settings::default_symbol_font);
        let available_theme_names =
            available_theme_names().unwrap_or_else(|_| vec!["dark".to_string()]);
        let available_font_names = available_font_names();
        let available_symbol_font_names = available_symbol_font_names();
        let mut active_theme_name = theme_settings.selected_theme.clone();
        if !available_theme_names
            .iter()
            .any(|theme_name| theme_name == &active_theme_name)
        {
            active_theme_name = "dark".to_string();
        }
        let _ = apply_saved_theme().or_else(|_| {
            load_theme_palette(&active_theme_name).inspect(|palette| {
                crate::gui::theme::apply_theme_palette(palette.clone());
            })
        });

        let manager = load_taskbar(&default_taskbar_path).unwrap_or_default();
        let draft_available_tags = manager.available_tags.clone();
        let draft_filter_tags = manager.active_filter_tags.clone();
        let draft_importance_filter = manager.active_importance_filter.clone();
        let draft_urgency_filter = manager.active_urgency_filter.clone();
        let draft_state_filter = manager.active_state_filter.clone();
        let draft_pinned_filter = manager.active_pinned_filter.clone();

        Gui {
            manager,
            collapsed: std::collections::HashSet::new(),
            side_panel: None,
            show_settings_menu: false,
            show_filter_menu: false,
            active_theme_name: active_theme_name.clone(),
            active_font_name: active_font_name.clone(),
            active_font,
            active_symbol_font_name: active_symbol_font_name.clone(),
            active_symbol_font,
            show_details_aside: theme_settings.show_details_aside,
            draft_theme_name: active_theme_name,
            draft_font_name: active_font_name,
            draft_symbol_font_name: active_symbol_font_name,
            draft_show_details_aside: theme_settings.show_details_aside,
            available_theme_names,
            available_font_names,
            available_symbol_font_names,
            settings_status: None,
            settings_confirm_clear_all: false,
            settings_confirm_clear_data_and_exit: false,
            draft_filter_tags,
            draft_importance_filter,
            draft_urgency_filter,
            draft_state_filter,
            draft_pinned_filter,
            task_file_path: default_taskbar_path,
            hovered_task: None,
            detail_name: text_editor::Content::new(),
            detail_description: text_editor::Content::new(),
            detail_description_focused: false,
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

    pub(super) fn view_main(&self) -> Element<'_, Message> {
        let mut main_row = Row::new()
            .spacing(16)
            .align_items(Alignment::Start)
            .push(self.view_task_list());

        if self.show_details_aside {
            if let Some(panel) = self.side_panel {
                match panel {
                    SidePanel::Detail(task_id) => {
                        if let Some(task) = self.manager.root.search_by_id_ref(task_id) {
                            main_row = main_row.push(self.view_detail(task, false));
                        }
                    }
                    SidePanel::Create(parent_id) => {
                        main_row = main_row.push(self.view_create_task(parent_id, false));
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

        if self.show_settings_menu {
            return Modal::new(content, Some(self.view_settings_modal()))
                .backdrop(Message::CloseSettingsMenu)
                .on_esc(Message::CloseSettingsMenu)
                .into();
        }

        if self.show_filter_menu {
            return Modal::new(content, Some(self.view_filter_modal()))
                .backdrop(Message::CancelFilterSelection)
                .on_esc(Message::CancelFilterSelection)
                .into();
        }

        if !self.show_details_aside {
            if let Some(overlay) = self.view_side_panel_overlay() {
                return Modal::new(content, Some(overlay))
                    .backdrop(Message::CloseDetail)
                    .on_esc(Message::CloseDetail)
                    .into();
            }
        }

        content.into()
    }

    pub(super) fn persist_changes(&self) {
        if let Err(error) = save_taskbar(&self.task_file_path, &self.manager) {
            eprintln!("Failed to save taskbar: {error}");
        }
    }

    pub(super) fn save_task_file_as(&mut self) {
        let dialog = FileDialog::new()
            .add_filter("Task files", &["json"])
            .set_file_name(
                self.task_file_path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or(DEFAULT_TASKBAR_FILE_NAME),
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

    pub(super) fn load_task_file_from(&mut self) {
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

    pub(super) fn sync_detail_content(&mut self) {
        let Some(panel) = self.side_panel else {
            self.reset_detail_draft();
            return;
        };

        if let SidePanel::Create(_) = panel {
            self.reset_detail_draft();
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
        self.detail_description_focused = false;
        self.draft_state = task.state.clone();
        self.draft_pinned = task.pinned;
        self.draft_tags = task.tags.clone();
        self.draft_available_tags = self.manager.available_tags.clone();
        for tag in &task.tags {
            if !self
                .draft_available_tags
                .iter()
                .any(|existing| existing == tag)
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
        self.active_date_panel = None;
        self.date_input_value.clear();
        self.clear_date_picker_state();
        self.delete_confirmation_for = None;
    }

    pub(super) fn date_value(&self, field: DateField) -> Option<DateTime<Utc>> {
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
                format!("Due: {}", crate::gui::theme::format_date(value))
            }
            (DateField::DueDate, None) => "Due: None".to_string(),
            (DateField::CompletedAt, Some(value)) => {
                format!("Completed: {}", crate::gui::theme::format_date(value))
            }
            (DateField::CompletedAt, None) => "Completed: None".to_string(),
        }
    }

    fn editor_text(content: &text_editor::Content) -> String {
        content.text().trim().to_string()
    }

    pub(super) fn reset_detail_draft(&mut self) {
        self.detail_name = text_editor::Content::with_text("New Task");
        self.detail_description = text_editor::Content::new();
        self.detail_description_focused = false;
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

    pub(super) fn clear_date_picker_state(&mut self) {
        self.date_selected_year = None;
        self.date_selected_month = None;
        self.date_selected_day = None;
        self.date_selected_hour = None;
        self.date_selected_minute = None;
    }

    pub(super) fn sync_date_picker_state(&mut self, value: DateTime<Utc>) {
        self.date_selected_year = Some(value.year());
        self.date_selected_month = Some(value.month());
        self.date_selected_day = Some(value.day());
        self.date_selected_hour = Some(value.hour());
        self.date_selected_minute = Some(value.minute());
    }

    pub(super) fn sync_text_from_picker_state(&mut self) {
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
        true
    }

    pub fn task_has_tag(&self, tag: &str) -> bool {
        self.draft_tags.iter().any(|existing| existing == tag)
    }

    pub fn can_toggle_tag(&self, tag: &str) -> bool {
        !tag.trim().is_empty()
    }

    pub fn common_tag_suggestions(&self) -> Vec<String> {
        self.manager
            .most_common_tags(5)
            .into_iter()
            .filter(|tag| !tag.trim().is_empty())
            .collect()
    }

    pub(super) fn clamp_selected_day(&mut self) {
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

    pub(super) fn save_detail(&mut self) {
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

    pub(super) fn delete_task(&mut self, id: u32) {
        if self.manager.delete_task(id).is_ok() {
            self.collapsed.remove(&id);
            self.delete_confirmation_for = None;
            if self.side_panel == Some(SidePanel::Detail(id)) {
                self.side_panel = None;
            }
            self.persist_changes();
            self.sync_detail_content();
        }
    }

    pub(super) fn save_settings(&mut self) {
        match load_theme_palette(&self.draft_theme_name) {
            Ok(palette) => {
                crate::gui::theme::apply_theme_palette(palette);
                self.active_theme_name = self.draft_theme_name.clone();
                self.active_font_name = normalize_font_name(&self.draft_font_name);
                self.active_font = font_option(&self.active_font_name)
                    .map(|option| option.font)
                    .unwrap_or_else(crate::gui::settings::default_font);
                self.active_symbol_font_name =
                    normalize_symbol_font_name(&self.draft_symbol_font_name);
                self.active_symbol_font = symbol_font_option(&self.active_symbol_font_name)
                    .map(|option| option.font)
                    .unwrap_or_else(crate::gui::settings::default_symbol_font);
                self.show_details_aside = self.draft_show_details_aside;
                let settings = GuiSettings {
                    selected_theme: self.active_theme_name.clone(),
                    selected_font: self.active_font_name.clone(),
                    selected_symbol_font: self.active_symbol_font_name.clone(),
                    show_details_aside: self.show_details_aside,
                };
                match save_gui_settings(&settings) {
                    Ok(()) => {
                        self.available_theme_names = available_theme_names()
                            .unwrap_or_else(|_| vec![self.active_theme_name.clone()]);
                        self.available_font_names = available_font_names();
                        self.available_symbol_font_names = available_symbol_font_names();
                        self.settings_status = Some("Settings saved.".to_string());
                        self.show_settings_menu = false;
                        self.settings_confirm_clear_all = false;
                        self.settings_confirm_clear_data_and_exit = false;
                    }
                    Err(error) => {
                        self.settings_status = Some(error);
                    }
                }
            }
            Err(error) => {
                self.settings_status = Some(error);
            }
        }
    }

    pub(super) fn clear_all_data_and_exit(&self) -> Command<Message> {
        match crate::app_paths::clear_app_data() {
            Ok(()) => {
                std::process::exit(0);
            }
            Err(error) => {
                eprintln!("Failed to clear app data: {error}");
                Command::none()
            }
        }
    }

    pub(super) fn handle_escape_shortcut(&mut self) {
        if self.show_settings_menu {
            let _ = self.handle_message(Message::CloseSettingsMenu);
            return;
        }

        if self.show_filter_menu {
            let _ = self.handle_message(Message::CancelFilterSelection);
            return;
        }

        if self.active_date_panel.is_some() {
            let _ = self.handle_message(Message::ToggleDatePanel(
                self.active_date_panel.expect("date panel checked as some"),
            ));
            return;
        }

        if matches!(
            self.side_panel,
            Some(SidePanel::Detail(_) | SidePanel::Create(_))
        ) {
            let _ = self.handle_message(Message::CloseDetail);
        }
    }

    pub(super) fn handle_submit_shortcut(&mut self) {
        if self.show_settings_menu {
            let _ = self.handle_message(Message::SaveSettings);
            return;
        }

        if self.show_filter_menu {
            let _ = self.handle_message(Message::ApplyFilterSelection);
            return;
        }

        if self.active_date_panel.is_some() {
            let _ = self.handle_message(Message::ApplyDateSelection);
            return;
        }

        if matches!(
            self.side_panel,
            Some(SidePanel::Detail(_) | SidePanel::Create(_))
        ) {
            let _ = self.handle_message(Message::SaveDetail);
        }
    }
}
