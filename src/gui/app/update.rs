use chrono::{DateTime, NaiveDateTime, Utc};
use iced::widget::text_editor;
use iced::Command;

use crate::gui::settings::{
    available_font_names, available_symbol_font_names, available_theme_names,
};
use crate::tasks::{
    ImportanceFilter, PinnedFilter, StateFilter, TaskImportance, TaskManager, TaskUrgency,
    UrgencyFilter,
};

use super::{DateField, Gui, Message, SidePanel};

impl Gui {
    pub(super) fn handle_message(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleCollapse(id) => {
                if self.collapsed.contains(&id) {
                    self.collapsed.remove(&id);
                } else {
                    self.collapsed.insert(id);
                }
            }
            Message::ToggleDetail(id) => {
                if self.side_panel == Some(SidePanel::Detail(id)) {
                    self.side_panel = None;
                } else {
                    self.side_panel = Some(SidePanel::Detail(id));
                }
                self.sync_detail_content();
            }
            Message::OpenCreateRoot => {
                self.side_panel = Some(SidePanel::Create(None));
                self.sync_detail_content();
            }
            Message::OpenCreateChild(id) => {
                self.side_panel = Some(SidePanel::Create(Some(id)));
                self.sync_detail_content();
            }
            Message::ToggleSettingsMenu => {
                self.show_settings_menu = !self.show_settings_menu;
                self.show_filter_menu = false;
                if self.show_settings_menu {
                    self.available_theme_names = available_theme_names()
                        .unwrap_or_else(|_| vec![self.active_theme_name.clone()]);
                    self.available_font_names = available_font_names();
                    self.available_symbol_font_names = available_symbol_font_names();
                    self.draft_theme_name = self.active_theme_name.clone();
                    self.draft_font_name = self.active_font_name.clone();
                    self.draft_symbol_font_name = self.active_symbol_font_name.clone();
                    self.draft_show_details_aside = self.show_details_aside;
                    self.settings_status = None;
                    self.settings_confirm_clear_all = false;
                    self.settings_confirm_clear_data_and_exit = false;
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
                self.settings_confirm_clear_data_and_exit = false;
            }
            Message::SelectFont(font_name) => {
                self.draft_font_name = font_name;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = false;
            }
            Message::SelectSymbolFont(font_name) => {
                self.draft_symbol_font_name = font_name;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = false;
            }
            Message::ToggleShowDetailsAside(value) => {
                self.draft_show_details_aside = value;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = false;
            }
            Message::SaveSettings => {
                self.save_settings();
            }
            Message::CloseSettingsMenu => {
                self.show_settings_menu = false;
                self.draft_theme_name = self.active_theme_name.clone();
                self.draft_font_name = self.active_font_name.clone();
                self.draft_symbol_font_name = self.active_symbol_font_name.clone();
                self.draft_show_details_aside = self.show_details_aside;
                self.settings_status = None;
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = false;
            }
            Message::SaveTaskFileAs => {
                self.save_task_file_as();
            }
            Message::LoadTaskFileFrom => {
                self.load_task_file_from();
            }
            Message::RequestClearAllTasks => {
                self.settings_confirm_clear_all = true;
                self.settings_confirm_clear_data_and_exit = false;
                self.settings_status =
                    Some("Click 'Confirm Clear' to remove all tasks.".to_string());
            }
            Message::ConfirmClearAllTasks => {
                self.manager = TaskManager::new();
                self.collapsed.clear();
                self.side_panel = None;
                self.hovered_task = None;
                self.delete_confirmation_for = None;
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = false;
                self.persist_changes();
                self.sync_detail_content();
                self.settings_status = Some(format!(
                    "Cleared all tasks and saved {}.",
                    self.task_file_path.display()
                ));
            }
            Message::RequestClearAllDataAndExit => {
                self.settings_confirm_clear_all = false;
                self.settings_confirm_clear_data_and_exit = true;
                self.settings_status = Some(
                    "Click 'Confirm Exit' to permanently delete all app data and quit.".to_string(),
                );
            }
            Message::ConfirmClearAllDataAndExit => {
                return self.clear_all_data_and_exit();
            }
            Message::HoverTaskEnter(id) => {
                self.hovered_task = Some(id);
            }
            Message::HoverTaskExit(id) => {
                if self.hovered_task == Some(id) {
                    self.hovered_task = None;
                }
            }
            Message::TogglePinned(id) => {
                if self.side_panel == Some(SidePanel::Detail(id))
                    || (id == 0 && matches!(self.side_panel, Some(SidePanel::Create(_))))
                {
                    self.draft_pinned = !self.draft_pinned;
                } else {
                    let _ = self.manager.toggle_task_pinned(id);
                    self.persist_changes();
                    self.sync_detail_content();
                }
                self.delete_confirmation_for = None;
            }
            Message::SelectState(id, new_state) => {
                if self.side_panel == Some(SidePanel::Detail(id))
                    || (id == 0 && matches!(self.side_panel, Some(SidePanel::Create(_))))
                {
                    self.draft_state = new_state;
                } else {
                    let _ = self.manager.set_task_state(id, new_state);
                    self.persist_changes();
                    self.sync_detail_content();
                }
                self.delete_confirmation_for = None;
            }
            Message::UndoLastChange => {
                if self.manager.undo_last_change().is_ok() {
                    self.persist_changes();
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
                if !tag.is_empty() {
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
                    let default_value = self.date_value(field).unwrap_or_else(Utc::now);
                    self.active_date_panel = Some(field);
                    self.date_input_value = crate::gui::theme::format_date(default_value);
                    self.sync_date_picker_state(default_value);
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
                    self.active_date_panel = None;
                    self.date_input_value.clear();
                    self.clear_date_picker_state();
                }
                self.delete_confirmation_for = None;
            }
            Message::DetailNameChanged(value) => {
                self.detail_name = text_editor::Content::with_text(&value);
                self.detail_description_focused = false;
                self.delete_confirmation_for = None;
            }
            Message::DetailDescriptionAction(action) => {
                self.detail_description.perform(action);
                self.detail_description_focused = true;
                self.delete_confirmation_for = None;
            }
            Message::SelectAllDetailDescription => {
                if self.detail_description_focused
                    && matches!(
                        self.side_panel,
                        Some(SidePanel::Detail(_) | SidePanel::Create(_))
                    )
                {
                    self.detail_description.perform(text_editor::Action::Move(
                        text_editor::Motion::DocumentStart,
                    ));
                    self.detail_description.perform(text_editor::Action::Select(
                        text_editor::Motion::DocumentEnd,
                    ));
                }
                self.delete_confirmation_for = None;
            }
            Message::CloseDetail => {
                self.side_panel = None;
                self.detail_description_focused = false;
                self.sync_detail_content();
            }
            Message::SaveDetail => {
                self.save_detail();
            }
            Message::TriggerEscapeShortcut => {
                self.handle_escape_shortcut();
            }
            Message::TriggerSubmitShortcut => {
                self.handle_submit_shortcut();
            }
        }

        Command::none()
    }
}
