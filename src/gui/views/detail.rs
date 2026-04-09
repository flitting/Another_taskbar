use iced::widget::{
    pick_list, text_editor, text_input, Button, Column, Container, Row, Scrollable, Space, Text,
};
use iced::{Alignment, Element, Length};

use crate::gui::theme::{
    container_highlight_style, container_secondary_style, container_tertiary_style,
    dark_pick_list_style, dark_text_input_style, detail_text_editor_style, modal_backdrop_style,
    text_primary_container_style, ButtonSurface,
};
use crate::symbols::{SYMBOL_ADD, SYMBOL_PIN};
use crate::tasks::Task;

use super::super::app::{DateField, DetailField, Gui, Message};

impl Gui {
    pub fn view_detail<'a>(&'a self, task: &'a Task) -> Element<'a, Message> {
        let has_subtasks = !task.subtasks.is_empty();
        let header = Column::new()
            .spacing(8)
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push_maybe(self.draft_pinned.then(|| Text::new(SYMBOL_PIN).size(20)))
                    .push(
                        text_input("Task name", &self.detail_name.text())
                            .on_input(Message::DetailNameChanged)
                            .padding([6, 0])
                            .size(28)
                            .style(dark_text_input_style())
                            .width(Length::Fill),
                    )
                    .push(Space::with_width(Length::Fill)),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(self.view_action_button(
                        self.state_button_label(),
                        14,
                        Some(Message::ToggleStateMenu(task.id)),
                        ButtonSurface::Highlight,
                        "Choose the current state for this task.",
                    ))
                    .push(self.view_action_button(
                        format!("{SYMBOL_PIN} Pinned"),
                        16,
                        Some(Message::TogglePinned(task.id)),
                        if self.draft_pinned {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Pin or unpin this task.",
                    ))
                    .push_maybe(self.can_undo().then(|| {
                        self.view_action_button(
                            "Undo",
                            14,
                            Some(Message::UndoLastChange),
                            ButtonSurface::Tertiary,
                            "Undo the most recent saved change.",
                        )
                    })),
            );

        let mut detail_col = Column::new().spacing(16).push(header).push(
            Column::new()
                .spacing(6)
                .push(Text::new("Description").size(14))
                .push(
                    Container::new(
                        text_editor(&self.detail_description)
                            .on_action(|action| {
                                Message::DetailTextAction(DetailField::Description, action)
                            })
                            .padding(0)
                            .style(detail_text_editor_style()),
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .style(container_tertiary_style),
                ),
        );

        detail_col =
            detail_col.push(
                Column::new()
                    .spacing(8)
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button(
                                self.urgency_label(),
                                Message::CycleUrgency,
                            ))
                            .push(self.view_detail_meta_button(
                                self.importance_label(),
                                Message::CycleImportance,
                            )),
                    )
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button(
                                self.date_button_label(DateField::DueDate),
                                Message::ToggleDatePanel(DateField::DueDate),
                            ))
                            .push(self.view_detail_meta_button(
                                self.date_button_label(DateField::CompletedAt),
                                Message::ToggleDatePanel(DateField::CompletedAt),
                            )),
                    ),
            );

        detail_col = detail_col.push(self.view_tag_picker());

        detail_col = detail_col
            .push(Column::new().spacing(10).push_maybe(self.view_date_panel()))
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
                        Text::new(self.detail_dates.text())
                            .size(12)
                            .width(Length::Shrink),
                    ),
            );

        Container::new(
            Scrollable::new(Container::new(detail_col).padding(16).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
        )
        .padding(16)
        .height(Length::Fill)
        .style(container_secondary_style)
        .into()
    }

    pub fn view_create_task<'a>(&'a self, parent_id: Option<u32>) -> Element<'a, Message> {
        let title = match parent_id {
            Some(_) => "New Subtask",
            None => "New Task",
        };

        let detail_col = Column::new()
            .spacing(16)
            .push(Text::new(title).size(28))
            .push(
                Container::new(
                    text_editor(&self.detail_name)
                        .on_action(|action| Message::DetailTextAction(DetailField::Name, action))
                        .padding(0)
                        .style(detail_text_editor_style()),
                )
                .padding([8, 10])
                .width(Length::Fill)
                .style(container_tertiary_style),
            )
            .push(
                Column::new()
                    .spacing(6)
                    .push(Text::new("Description").size(14))
                    .push(
                        Container::new(
                            text_editor(&self.detail_description)
                                .on_action(|action| {
                                    Message::DetailTextAction(DetailField::Description, action)
                                })
                                .padding(0)
                                .style(detail_text_editor_style()),
                        )
                        .padding(12)
                        .width(Length::Fill)
                        .style(container_tertiary_style),
                    ),
            )
            .push(
                Column::new()
                    .spacing(8)
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button(
                                self.urgency_label(),
                                Message::CycleUrgency,
                            ))
                            .push(self.view_detail_meta_button(
                                self.importance_label(),
                                Message::CycleImportance,
                            )),
                    )
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(self.view_detail_meta_button(
                                self.date_button_label(DateField::DueDate),
                                Message::ToggleDatePanel(DateField::DueDate),
                            ))
                            .push(self.view_detail_meta_button(
                                self.date_button_label(DateField::CompletedAt),
                                Message::ToggleDatePanel(DateField::CompletedAt),
                            )),
                    )
                    .push_maybe(self.view_date_panel()),
            )
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

        Container::new(
            Scrollable::new(Container::new(detail_col).padding(16).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
        )
        .padding(16)
        .height(Length::Fill)
        .style(container_secondary_style)
        .into()
    }

    pub fn view_side_panel_overlay(&self) -> Option<Element<'_, Message>> {
        let panel = self.side_panel?;

        let modal: Element<'_, Message> = match panel {
            super::super::app::SidePanel::Detail(task_id) => {
                let task = self.manager.root.search_by_id_ref(task_id)?;
                Container::new(self.view_detail(task))
                    .width(Length::Fixed(640.0))
                    .height(Length::FillPortion(4))
                    .into()
            }
            super::super::app::SidePanel::Create(parent_id) => {
                Container::new(self.view_create_task(parent_id))
                    .width(Length::Fixed(640.0))
                    .height(Length::FillPortion(4))
                    .into()
            }
        };

        Some(
            Container::new(
                Column::new()
                    .push(Space::with_height(Length::FillPortion(1)))
                    .push(
                        Row::new()
                            .align_items(Alignment::Center)
                            .push(Space::with_width(Length::Fill))
                            .push(modal)
                            .push(Space::with_width(Length::Fill)),
                    )
                    .push(Space::with_height(Length::FillPortion(1))),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(modal_backdrop_style)
            .into(),
        )
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

        self.view_action_button(
            label,
            13,
            Some(message),
            ButtonSurface::Tertiary,
            explanation,
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
        let mut tags_row = Row::new().spacing(8).align_items(Alignment::Center);

        for tag in &self.draft_available_tags {
            tags_row = tags_row.push(self.view_action_button(
                tag.clone(),
                13,
                Some(Message::ToggleTaskTag(tag.clone())),
                if self.task_has_tag(tag) {
                    ButtonSurface::Highlight
                } else {
                    ButtonSurface::Tag
                },
                format!("Toggle the '{tag}' tag on this task."),
            ));
        }

        tags_row = tags_row.push(self.view_action_button(
            SYMBOL_ADD,
            16,
            self.can_add_more_tags().then_some(Message::ToggleTagEditor),
            ButtonSurface::Tertiary,
            "Show the tag input and tag list.",
        ));

        let mut content = Column::new()
            .spacing(10)
            .push(Text::new("Tags").size(14))
            .push(tags_row);

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

            if !self.draft_available_tags.is_empty() {
                let mut existing_tags = Row::new().spacing(8);
                for tag in &self.draft_available_tags {
                    existing_tags = existing_tags.push(self.view_action_button(
                        tag.clone(),
                        12,
                        Some(Message::ToggleTaskTag(tag.clone())),
                        if self.task_has_tag(tag) {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tag
                        },
                        format!("Toggle the '{tag}' tag on this task."),
                    ));
                }
                content = content.push(existing_tags);
            }
        }

        Container::new(content)
            .padding(12)
            .width(Length::Fill)
            .style(container_tertiary_style)
            .into()
    }
}
