use iced::widget::{checkbox, pick_list, Column, Container, Row, Space, Text};
use iced::{Alignment, Element, Length};

use crate::gui::settings::{theme_path, THEMES_DIR};
use crate::gui::theme::{
    container_menu_bg_light_style, current_theme_palette, dark_pick_list_style,
    modal_backdrop_style, ButtonSurface,
};

use super::super::app::{Gui, Message};

impl Gui {
    pub fn view_settings_modal(&self) -> Element<'_, Message> {
        let palette = current_theme_palette();
        let mut content = Column::new()
            .spacing(14)
            .push(Text::new("Settings").size(22))
            .push(Text::new("Theme").size(14))
            .push(
                pick_list(
                    self.available_theme_names.clone(),
                    Some(self.draft_theme_name.clone()),
                    Message::SelectTheme,
                )
                .placeholder("Select theme")
                .padding([8, 10])
                .style(dark_pick_list_style())
                .width(Length::Fill),
            )
            .push(
                Text::new(format!(
                    "Built-in and custom theme files live in '{}'. Current file: {}",
                    THEMES_DIR,
                    theme_path(&self.draft_theme_name).display()
                ))
                .size(12),
            )
            .push(Text::new(format!("Active palette: {}", palette.name)).size(12))
            .push(Text::new("Layout").size(14))
            .push(
                checkbox("Show details aside", self.draft_show_details_aside)
                    .on_toggle(Message::ToggleShowDetailsAside)
                    .size(16)
                    .spacing(10),
            )
            .push(
                Text::new(
                    "When enabled, detail and create panels stay docked on the right. Turn it off to open them as floating windows.",
                )
                .size(12),
            )
            .push(Text::new("Task File").size(14))
            .push(Text::new(format!("Path: {}", self.task_file_path.display())).size(12))
            .push(
                Row::new()
                    .spacing(8)
                    .push(self.view_action_button(
                        "Load...",
                        14,
                        Some(Message::LoadTaskFileFrom),
                        ButtonSurface::Tertiary,
                        "Open a file picker and load tasks from a JSON file.",
                    ))
                    .push(self.view_action_button(
                        "Save As...",
                        14,
                        Some(Message::SaveTaskFileAs),
                        ButtonSurface::Tertiary,
                        "Open a file picker and save the current tasks to a JSON file.",
                    )),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .push(self.view_action_button(
                        "Clear All",
                        14,
                        Some(Message::RequestClearAllTasks),
                        ButtonSurface::Highlight,
                        "Request deletion of every task in the currently selected task file.",
                    ))
                    .push_maybe(self.settings_confirm_clear_all.then(|| {
                        self.view_action_button(
                            "Confirm Clear",
                            14,
                            Some(Message::ConfirmClearAllTasks),
                            ButtonSurface::Highlight,
                            "Delete every task and save the empty selected task file.",
                        )
                    })),
            );

        if let Some(status) = &self.settings_status {
            content = content.push(Text::new(status).size(12));
        }

        content = content.push(
            Row::new()
                .spacing(8)
                .push(self.view_action_button(
                    "Save",
                    14,
                    Some(Message::SaveSettings),
                    ButtonSurface::Highlight,
                    "Save the selected theme and close this settings panel.",
                ))
                .push(self.view_action_button(
                    "Close",
                    14,
                    Some(Message::CloseSettingsMenu),
                    ButtonSurface::Tertiary,
                    "Close the settings panel without saving theme changes.",
                )),
        );

        Container::new(content)
            .padding(18)
            .width(Length::Fixed(360.0))
            .style(container_menu_bg_light_style)
            .into()
    }

    pub fn view_settings_overlay(&self) -> Element<'_, Message> {
        Container::new(
            Column::new()
                .push(Space::with_height(Length::FillPortion(1)))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(Space::with_width(Length::Fill))
                        .push(self.view_settings_modal())
                        .push(Space::with_width(Length::Fill)),
                )
                .push(Space::with_height(Length::FillPortion(2))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(modal_backdrop_style)
        .into()
    }
}
