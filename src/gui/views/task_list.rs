use iced::widget::{mouse_area, text_input, Column, Container, Row, Scrollable, Space, Text};
use iced::{Alignment, Element, Length};

use crate::gui::theme::{
    all_task_states, container_menu_bg_light_style, container_pinned_style,
    container_secondary_style, container_tertiary_style, current_theme_palette,
    dark_text_input_style, task_state_icon, ButtonSurface,
};
use crate::symbols::{SYMBOL_ADD, SYMBOL_COLLAPSED, SYMBOL_EXPANDED, SYMBOL_PIN, SYMBOL_SETTINGS};
use crate::tasks::{Task, TaskImportance, TaskUrgency};

use super::super::app::{Gui, Message};

impl Gui {
    pub fn view_task_list(&self) -> Element<'_, Message> {
        let header = Row::new()
            .align_items(Alignment::Center)
            .push(Text::new("Tasks").size(32))
            .push(Space::with_width(Length::Fill))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(
                        text_input("Search tasks", &self.manager.active_search_query)
                            .on_input(Message::SearchQueryChanged)
                            .padding([8, 10])
                            .size(14)
                            .style(dark_text_input_style())
                            .width(Length::Fixed(180.0)),
                    )
                    .push(self.view_action_button(
                        "Clear Search",
                        14,
                        self.manager.has_active_search().then_some(Message::ClearSearchQuery),
                        if self.manager.has_active_search() {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Clear the current search text and show all matching filter results again.",
                    ))
                    .push(self.view_action_button(
                        SYMBOL_SETTINGS,
                        18,
                        Some(Message::ToggleSettingsMenu),
                        ButtonSurface::Tertiary,
                        "Open the settings menu.",
                    ))
                    .push(self.view_action_button(
                        "Filter",
                        14,
                        Some(Message::ToggleFilterMenu),
                        if self.manager.has_active_filters() {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Open task filters for tags, importance, urgency, and pinned state.",
                    ))
                    .push(self.view_action_button(
                        "Undo",
                        14,
                        self.can_undo().then_some(Message::UndoLastChange),
                        if self.can_undo() {
                            ButtonSurface::Highlight
                        } else {
                            ButtonSurface::Tertiary
                        },
                        "Undo the most recent saved change.",
                    ))
                    .push(self.view_action_button(
                        SYMBOL_ADD,
                        20,
                        Some(Message::OpenCreateRoot),
                        ButtonSurface::Highlight,
                        "Create a new top-level task.",
                    )),
            );

        let mut task_column = Column::new().spacing(8).push(header);

        let filtered_tasks = self.manager.filtered_tasks();

        for task in filtered_tasks {
            task_column = task_column.push(self.render_task_recursive(task, 0));
        }

        let scrollable_content = Scrollable::new(task_column)
            .height(Length::Fill)
            .width(Length::FillPortion(2));

        Container::new(scrollable_content)
            .padding(16)
            .height(Length::Fill)
            .style(container_secondary_style)
            .into()
    }

    pub fn render_task_recursive(&self, task: Task, depth: usize) -> Element<'static, Message> {
        let palette = current_theme_palette();
        let indent = depth * 24;
        let task_container_style = if task.pinned {
            container_pinned_style
        } else {
            container_tertiary_style
        };

        let mut task_row = Row::new().spacing(0).align_items(Alignment::Center);

        if indent > 0 {
            task_row = task_row.push(Space::with_width(Length::Fixed(indent as f32)));
        }

        let mut stripes = Row::new().spacing(3).align_items(Alignment::Center);
        match task.importance {
            Some(TaskImportance::High) => {
                stripes = stripes.push(self.view_task_stripe(palette.importance_high_stripe));
            }
            Some(TaskImportance::Low) => {
                stripes = stripes.push(self.view_task_stripe(palette.importance_low_stripe));
            }
            None => {}
        }
        match task.urgency {
            Some(TaskUrgency::High) => {
                stripes = stripes.push(self.view_task_stripe(palette.urgency_high_stripe));
            }
            Some(TaskUrgency::Low) => {
                stripes = stripes.push(self.view_task_stripe(palette.urgency_low_stripe));
            }
            None => {}
        }

        let mut task_container_row = Row::new()
            .spacing(0)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        if !task.subtasks.is_empty() {
            let symbol = if self.collapsed.contains(&task.id) {
                SYMBOL_COLLAPSED
            } else {
                SYMBOL_EXPANDED
            };
            task_container_row = task_container_row.push(self.view_plain_button(
                symbol,
                16,
                Some(Message::ToggleCollapse(task.id)),
                palette.text_muted,
            ));
        } else {
            task_container_row = task_container_row.push(Space::with_width(Length::Fixed(40.0)));
        }

        let state_icon = task_state_icon(&task.state);
        task_container_row = task_container_row.push(self.view_plain_button(
            state_icon,
            18,
            Some(Message::ToggleStateMenu(task.id)),
            palette.text_muted,
        ));

        task_container_row = task_container_row.push(self.view_plain_button_fill(
            &task.name,
            16,
            Some(Message::ToggleDetail(task.id)),
            ButtonSurface::Highlight,
        ));

        if self.hovered_task == Some(task.id) {
            if !matches!(task.importance, None) || !matches!(task.urgency, None) {
                task_container_row =
                    task_container_row.push(stripes.push(Space::with_width(Length::Fixed(8.0))));
            }
            task_container_row = task_container_row.push(self.view_plain_button(
                SYMBOL_PIN,
                16,
                Some(Message::TogglePinned(task.id)),
                if task.pinned {
                    palette.text_primary
                } else {
                    palette.text_muted
                },
            ));
            task_container_row = task_container_row.push(self.view_plain_button(
                SYMBOL_ADD,
                18,
                Some(Message::OpenCreateChild(task.id)),
                palette.text_muted,
            ));
        } else if !matches!(task.importance, None) || !matches!(task.urgency, None) {
            task_container_row =
                task_container_row.push(stripes.push(Space::with_width(Length::Fixed(8.0))));
        }

        let task_row_surface = mouse_area(
            Container::new(task_container_row)
                .padding(6)
                .width(Length::Fill)
                .style(task_container_style),
        )
        .on_enter(Message::HoverTaskEnter(task.id))
        .on_exit(Message::HoverTaskExit(task.id));

        task_row = task_row.push(task_row_surface);

        let mut container_col = Column::new().spacing(4).push(task_row);

        if self.state_menu_for == Some(task.id) {
            let mut menu = Column::new().spacing(6).padding(8);

            for s in all_task_states() {
                let label = format!("{} {}", task_state_icon(&s), s);
                menu = menu.push(self.view_menu_button(
                    label,
                    Message::SelectState(task.id, s.clone()),
                    format!("Set this task to {}.", s),
                ));
            }

            container_col = container_col.push(
                Container::new(menu)
                    .padding(8)
                    .width(Length::Fixed(180.0))
                    .style(container_menu_bg_light_style),
            );
        }

        if !task.subtasks.is_empty() && !self.collapsed.contains(&task.id) {
            for sub in task.subtasks.clone() {
                container_col = container_col.push(self.render_task_recursive(sub, depth + 1));
            }
        }

        Container::new(container_col).width(Length::Fill).into()
    }

    fn view_task_stripe(&self, color: iced::Color) -> Element<'static, Message> {
        Container::new(Space::with_width(Length::Fixed(4.0)))
            .width(Length::Fixed(6.0))
            .height(Length::Fixed(28.0))
            .style(move |_: &iced::Theme| stripe_appearance(color))
            .into()
    }
}

fn stripe_appearance(color: iced::Color) -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        background: Some(iced::Background::Color(color)),
        ..Default::default()
    }
}
