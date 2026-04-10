use iced::widget::{
    pick_list, text_editor, text_input, Button, Column, Container, Row, Scrollable, Space, Text,
};
use iced::{Alignment, Element, Length};

use crate::gui::theme::{
    all_task_state_options, container_highlight_style, container_input_style,
    container_secondary_style, container_tertiary_style, dark_pick_list_style,
    dark_scrollable_style, dark_text_input_style, detail_text_editor_style,
    popup_window_inner_style, popup_window_style, task_state_option, text_primary_container_style,
    ButtonSurface,
};
use crate::symbols::{SYMBOL_ADD, SYMBOL_PIN};
use crate::tasks::Task;

use super::super::app::{DateField, Gui, Message};

const DESCRIPTION_EDITOR_HEIGHT: f32 = 148.0;
const DESCRIPTION_VISIBLE_LINES: usize = 6;
const TAG_ROW_WRAP_WIDTH: f32 = 560.0;
const TAG_BUTTON_BASE_WIDTH: f32 = 28.0;
const TAG_BUTTON_CHAR_WIDTH: f32 = 8.5;
const TAG_ROW_MAX_ITEMS: usize = 6;

impl Gui {
    pub fn view_detail<'a>(&'a self, task: &'a Task, floating: bool) -> Element<'a, Message> {
        let has_subtasks = !task.subtasks.is_empty();
        let action_width = Length::Fixed(152.0);
        let header = Column::new()
            .spacing(8)
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push_maybe(self.draft_pinned.then(|| Text::new(SYMBOL_PIN).size(20)))
                    .push(self.view_task_name_editor())
                    .push(Space::with_width(Length::Fill))
                    .push(self.view_close_button()),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(
                        pick_list(
                            all_task_state_options(),
                            Some(task_state_option(&self.draft_state)),
                            move |selected| Message::SelectState(task.id, selected.state),
                        )
                        .padding([8, 12])
                        .text_size(14)
                        .style(dark_pick_list_style())
                        .width(action_width),
                    )
                    .push(self.view_action_button_with_width(
                        format!("{SYMBOL_PIN} Pinned"),
                        16,
                        Some(Message::TogglePinned(task.id)),
                        if self.draft_pinned {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Pin or unpin this task.",
                        action_width,
                    ))
                    .push_maybe(self.can_undo().then(|| {
                        self.view_action_button_with_width(
                            "Undo",
                            14,
                            Some(Message::UndoLastChange),
                            ButtonSurface::Tertiary,
                            "Undo the most recent saved change.",
                            action_width,
                        )
                    })),
            );

        let mut detail_col = Column::new().spacing(16).push(header).push(
            Column::new()
                .spacing(6)
                .push(Text::new("Description").size(14))
                .push(self.view_description_editor()),
        );

        detail_col = detail_col.push(
            Column::new()
                .spacing(8)
                .push(
                    Row::new()
                        .spacing(8)
                        .push(self.view_detail_meta_button_fill(
                            self.urgency_label(),
                            Message::CycleUrgency,
                        ))
                        .push(self.view_detail_meta_button_fill(
                            self.importance_label(),
                            Message::CycleImportance,
                        )),
                )
                .push(
                    Row::new()
                        .spacing(8)
                        .push(self.view_detail_meta_button_fill(
                            self.date_button_label(DateField::DueDate),
                            Message::ToggleDatePanel(DateField::DueDate),
                        ))
                        .push(self.view_detail_meta_button_fill(
                            self.date_button_label(DateField::CompletedAt),
                            Message::ToggleDatePanel(DateField::CompletedAt),
                        )),
                ),
        );

        detail_col = detail_col
            .push_maybe(self.view_date_panel())
            .push(self.view_tag_picker())
            .push_maybe(
                (self.delete_confirmation_for == Some(task.id) && has_subtasks).then(|| {
                    Container::new(
                        Column::new()
                            .spacing(8)
                            .push(Text::new("Delete this task and all subtasks?").size(14))
                            .push(
                                Row::new()
                                    .spacing(8)
                                    .push(self.view_detail_meta_button(
                                        "Delete all".to_string(),
                                        Message::ConfirmDelete(task.id),
                                    ))
                                    .push(self.view_detail_meta_button(
                                        "Cancel".to_string(),
                                        Message::CancelDelete,
                                    )),
                            ),
                    )
                    .padding(12)
                    .style(container_tertiary_style)
                }),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .push(self.view_action_button(
                        "Save",
                        14,
                        Some(Message::SaveDetail),
                        ButtonSurface::Highlight,
                        "Save all edits in this detail panel as one change.",
                    ))
                    .push(self.view_action_button(
                        "Delete",
                        14,
                        Some(if has_subtasks {
                            Message::RequestDelete(task.id)
                        } else {
                            Message::ConfirmDelete(task.id)
                        }),
                        ButtonSurface::Highlight,
                        "Delete this task. If it has subtasks, confirmation is required.",
                    ))
                    .push(self.view_action_button(
                        "Close",
                        14,
                        Some(Message::CloseDetail),
                        ButtonSurface::Highlight,
                        "Close the detail panel without saving pending edits.",
                    ))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(self.detail_timestamp_text(task))
                            .size(12)
                            .width(Length::Shrink),
                    ),
            );

        let mut surface = Container::new(
            Scrollable::new(Container::new(detail_col).padding(16).width(Length::Fill))
                .style(dark_scrollable_style())
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
        )
        .padding(16)
        .height(Length::Fill);

        if floating {
            surface = surface.style(popup_window_inner_style);
        } else {
            surface = surface.style(container_secondary_style);
        }

        surface.into()
    }

    pub fn view_create_task<'a>(
        &'a self,
        _parent_id: Option<u32>,
        floating: bool,
    ) -> Element<'a, Message> {
        let action_width = Length::Fixed(152.0);
        let detail_col = Column::new()
            .spacing(16)
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(self.view_task_name_editor())
                    .push(self.view_close_button()),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(
                        pick_list(
                            all_task_state_options(),
                            Some(task_state_option(&self.draft_state)),
                            move |selected| Message::SelectState(0, selected.state),
                        )
                        .padding([8, 12])
                        .text_size(14)
                        .style(dark_pick_list_style())
                        .width(action_width),
                    )
                    .push(self.view_action_button_with_width(
                        format!("{SYMBOL_PIN} Pinned"),
                        16,
                        Some(Message::TogglePinned(0)),
                        if self.draft_pinned {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Pin or unpin this task.",
                        action_width,
                    ))
                    .push_maybe(self.can_undo().then(|| {
                        self.view_action_button_with_width(
                            "Undo",
                            14,
                            Some(Message::UndoLastChange),
                            ButtonSurface::Tertiary,
                            "Undo the most recent saved change.",
                            action_width,
                        )
                    })),
            )
            .push(
                Column::new()
                    .spacing(6)
                    .push(Text::new("Description").size(14))
                    .push(self.view_description_editor()),
            )
            .push(
                Column::new()
                    .spacing(8)
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button_fill(
                                self.urgency_label(),
                                Message::CycleUrgency,
                            ))
                            .push(self.view_detail_meta_button_fill(
                                self.importance_label(),
                                Message::CycleImportance,
                            )),
                    )
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button_fill(
                                self.date_button_label(DateField::DueDate),
                                Message::ToggleDatePanel(DateField::DueDate),
                            ))
                            .push(self.view_detail_meta_button_fill(
                                self.date_button_label(DateField::CompletedAt),
                                Message::ToggleDatePanel(DateField::CompletedAt),
                            )),
                    ),
            )
            .push_maybe(self.view_date_panel())
            .push(self.view_tag_picker())
            .push(
                Row::new()
                    .spacing(8)
                    .push(
                        Container::new(
                            Button::new(
                                Container::new(Text::new("Save").size(14))
                                    .style(text_primary_container_style),
                            )
                            .padding([8, 12])
                            .on_press(Message::SaveDetail)
                            .style(iced::theme::Button::Text),
                        )
                        .style(container_highlight_style),
                    )
                    .push(
                        Container::new(
                            Button::new(
                                Container::new(Text::new("Close").size(14))
                                    .style(text_primary_container_style),
                            )
                            .padding([8, 12])
                            .on_press(Message::CloseDetail)
                            .style(iced::theme::Button::Text),
                        )
                        .style(container_highlight_style),
                    ),
            );

        let mut surface = Container::new(
            Scrollable::new(Container::new(detail_col).padding(16).width(Length::Fill))
                .style(dark_scrollable_style())
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
        )
        .padding(16)
        .height(Length::Fill);

        if floating {
            surface = surface.style(popup_window_inner_style);
        } else {
            surface = surface.style(container_secondary_style);
        }

        surface.into()
    }

    pub fn view_side_panel_overlay(&self) -> Option<Element<'_, Message>> {
        let panel = self.side_panel?;

        match panel {
            super::super::app::SidePanel::Detail(task_id) => {
                let task = self.manager.root.search_by_id_ref(task_id)?;
                Some(
                    Container::new(
                        Container::new(self.view_detail(task, true))
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .padding(1)
                    .width(Length::Fixed(760.0))
                    .height(Length::Fixed(700.0))
                    .style(popup_window_style)
                    .into(),
                )
            }
            super::super::app::SidePanel::Create(parent_id) => Some(
                Container::new(
                    Container::new(self.view_create_task(parent_id, true))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .padding(1)
                .width(Length::Fixed(760.0))
                .height(Length::Fixed(700.0))
                .style(popup_window_style)
                .into(),
            ),
        }
    }

    fn view_detail_meta_button<'a>(&self, label: String, message: Message) -> Element<'a, Message> {
        let explanation = match label.as_str() {
            "Apply" => "Apply the selected date and time.",
            "Clear" => "Clear the selected date field.",
            _ if label.starts_with("Due:") => "Open the due date picker.",
            _ if label.starts_with("Completed:") => "Open the completed date picker.",
            _ if label.contains("Urgency") => "Cycle through urgency values.",
            _ if label.contains("Importance") => "Cycle through importance values.",
            _ => "Open or change this setting.",
        };

        self.view_action_button(label, 13, Some(message), ButtonSurface::Meta, explanation)
    }

    fn view_detail_meta_button_fill<'a>(
        &self,
        label: String,
        message: Message,
    ) -> Element<'a, Message> {
        let explanation = match label.as_str() {
            "Apply" => "Apply the selected date and time.",
            "Clear" => "Clear the selected date field.",
            _ if label.starts_with("Due:") => "Open the due date picker.",
            _ if label.starts_with("Completed:") => "Open the completed date picker.",
            _ if label.contains("Urgency") => "Cycle through urgency values.",
            _ if label.contains("Importance") => "Cycle through importance values.",
            _ => "Open or change this setting.",
        };

        self.view_action_button_with_width(
            label,
            13,
            Some(message),
            ButtonSurface::Meta,
            explanation,
            Length::FillPortion(1),
        )
    }

    fn view_task_name_editor<'a>(&self) -> Element<'a, Message> {
        Container::new(
            text_input("Task name", &self.detail_name.text())
                .on_input(Message::DetailNameChanged)
                .padding([12, 14])
                .size(28)
                .style(dark_text_input_style())
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .into()
    }

    fn view_description_editor<'a>(&'a self) -> Element<'a, Message> {
        let editor = text_editor(&self.detail_description)
            .on_action(Message::DetailDescriptionAction)
            .padding([12, 12])
            .style(detail_text_editor_style());

        let content: Element<'a, Message> = if self.description_needs_internal_scroll() {
            Scrollable::new(editor.height(Length::Shrink))
                .style(dark_scrollable_style())
                .height(Length::Fixed(DESCRIPTION_EDITOR_HEIGHT))
                .into()
        } else {
            editor
                .height(Length::Fixed(DESCRIPTION_EDITOR_HEIGHT))
                .into()
        };

        Container::new(content)
            .width(Length::Fill)
            .style(container_input_style)
            .into()
    }

    fn description_needs_internal_scroll(&self) -> bool {
        self.detail_description.line_count() > DESCRIPTION_VISIBLE_LINES
    }

    fn view_close_button<'a>(&self) -> Element<'a, Message> {
        self.view_action_button(
            "X",
            14,
            Some(Message::CloseDetail),
            ButtonSurface::Tertiary,
            "Close this popup.",
        )
    }

    fn view_date_panel<'a>(&'a self) -> Option<Element<'a, Message>> {
        let field = self.active_date_panel?;
        let title = match field {
            DateField::DueDate => "Set Due Time",
            DateField::CompletedAt => "Set Completed Time",
        };

        Some(
            Container::new(
                Column::new()
                    .spacing(10)
                    .push(Text::new(title).size(14))
                    .push(
                        text_input("YYYY-MM-DD HH:MM", &self.date_input_value)
                            .on_input(Message::DateInputChanged)
                            .padding([8, 10])
                            .style(dark_text_input_style()),
                    )
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(
                                pick_list(
                                    self.year_options(),
                                    self.date_selected_year,
                                    Message::SelectDateYear,
                                )
                                .placeholder("Year")
                                .padding([8, 10])
                                .style(dark_pick_list_style())
                                .width(Length::FillPortion(1)),
                            )
                            .push(
                                pick_list(
                                    self.month_options(),
                                    self.date_selected_month,
                                    Message::SelectDateMonth,
                                )
                                .placeholder("Month")
                                .padding([8, 10])
                                .style(dark_pick_list_style())
                                .width(Length::FillPortion(1)),
                            )
                            .push(
                                pick_list(
                                    self.day_options(),
                                    self.date_selected_day,
                                    Message::SelectDateDay,
                                )
                                .placeholder("Day")
                                .padding([8, 10])
                                .style(dark_pick_list_style())
                                .width(Length::FillPortion(1)),
                            ),
                    )
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(
                                pick_list(
                                    self.hour_options(),
                                    self.date_selected_hour,
                                    Message::SelectDateHour,
                                )
                                .placeholder("Hour")
                                .padding([8, 10])
                                .style(dark_pick_list_style())
                                .width(Length::FillPortion(1)),
                            )
                            .push(
                                pick_list(
                                    self.minute_options(),
                                    self.date_selected_minute,
                                    Message::SelectDateMinute,
                                )
                                .placeholder("Mins")
                                .padding([8, 10])
                                .style(dark_pick_list_style())
                                .width(Length::FillPortion(2)),
                            ),
                    )
                    .push(Text::new("Use 24-hour time, exact to minutes.").size(12))
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button(
                                "Apply".to_string(),
                                Message::ApplyDateSelection,
                            ))
                            .push(self.view_detail_meta_button(
                                "Clear".to_string(),
                                Message::ClearDateSelection,
                            )),
                    ),
            )
            .padding(12)
            .width(Length::Fill)
            .style(container_tertiary_style)
            .into(),
        )
    }

    fn view_tag_picker<'a>(&self) -> Element<'a, Message> {
        let mut selected_tag_items = Vec::new();

        for tag in &self.draft_tags {
            selected_tag_items.push((
                self.view_action_button(
                    tag.clone(),
                    13,
                    Some(Message::ToggleTaskTag(tag.clone())),
                    ButtonSurface::TagActive,
                    format!("Remove the '{tag}' tag from this task."),
                ),
                Self::estimated_tag_button_width(tag),
            ));
        }

        selected_tag_items.push((
            self.view_action_button(
                SYMBOL_ADD,
                16,
                self.can_add_more_tags().then_some(Message::ToggleTagEditor),
                ButtonSurface::Tertiary,
                "Show the tag input and tag list.",
            ),
            Self::estimated_tag_button_width(SYMBOL_ADD),
        ));

        let mut content = Column::new()
            .spacing(10)
            .push(Self::view_wrapped_button_rows(selected_tag_items));

        if self.show_tag_editor {
            content = content.push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(
                        text_input("New tag", &self.tag_input_value)
                            .on_input(Message::TagInputChanged)
                            .padding([8, 10])
                            .style(dark_text_input_style()),
                    )
                    .push(self.view_action_button(
                        "Add",
                        13,
                        self.can_add_more_tags().then_some(Message::AddDraftTag),
                        ButtonSurface::Highlight,
                        "Add a new shared tag and select it for this task.",
                    )),
            );

            let common_tags = self.common_tag_suggestions();
            if !common_tags.is_empty() {
                let mut existing_tag_items = Vec::new();
                for tag in common_tags {
                    existing_tag_items.push((
                        self.view_action_button(
                            tag.clone(),
                            12,
                            self.can_toggle_tag(&tag)
                                .then_some(Message::ToggleTaskTag(tag.clone())),
                            if self.task_has_tag(&tag) {
                                ButtonSurface::TagActive
                            } else {
                                ButtonSurface::Tag
                            },
                            format!("Quick add the '{tag}' tag to this task."),
                        ),
                        Self::estimated_tag_button_width(&tag),
                    ));
                }
                content = content
                    .push(Text::new("Quick Add").size(12))
                    .push(Self::view_wrapped_button_rows(existing_tag_items));
            }
        }

        Column::new()
            .spacing(6)
            .push(Text::new("Tags").size(14))
            .push(
                Container::new(content)
                    .padding(12)
                    .width(Length::Fill)
                    .style(container_tertiary_style),
            )
            .into()
    }

    fn detail_timestamp_text(&self, task: &Task) -> String {
        format!(
            "Created: {}\nUpdated: {}",
            crate::gui::theme::format_date(task.times.created_at),
            crate::gui::theme::format_date(task.times.updated_at)
        )
    }

    fn estimated_tag_button_width(tag: &str) -> f32 {
        TAG_BUTTON_BASE_WIDTH + tag.chars().count() as f32 * TAG_BUTTON_CHAR_WIDTH
    }

    fn view_wrapped_button_rows<'a>(
        items: Vec<(Element<'a, Message>, f32)>,
    ) -> Element<'a, Message> {
        let mut rows: Vec<Vec<Element<'a, Message>>> = Vec::new();
        let mut current_row: Vec<Element<'a, Message>> = Vec::new();
        let mut current_width = 0.0;

        for (item, width) in items {
            let spacing = if current_row.is_empty() { 0.0 } else { 8.0 };

            if !current_row.is_empty()
                && (current_width + spacing + width > TAG_ROW_WRAP_WIDTH
                    || current_row.len() >= TAG_ROW_MAX_ITEMS)
            {
                rows.push(current_row);
                current_row = Vec::new();
                current_width = 0.0;
            }

            current_width += if current_row.is_empty() {
                width
            } else {
                spacing + width
            };
            current_row.push(item);
        }

        if !current_row.is_empty() {
            rows.push(current_row);
        }

        rows.into_iter()
            .fold(Column::new().spacing(8), |column, row_items| {
                let row = row_items.into_iter().fold(
                    Row::new().spacing(8).align_items(Alignment::Center),
                    |row, item| row.push(item),
                );
                column.push(row)
            })
            .into()
    }
}
