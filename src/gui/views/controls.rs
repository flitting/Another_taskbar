use iced::alignment::Horizontal;
use iced::widget::{tooltip, Button, Container, Text};
use iced::{Element, Length};

use crate::gui::theme::{
    action_button_style, inline_button_style, tooltip_container_style, ButtonSurface,
};

use super::super::app::{Gui, Message};

impl Gui {
    pub fn symbol_text<'a, S: Into<String>>(&self, label: S, size: u16) -> Text<'a> {
        Text::new(label.into()).font(self.active_symbol_font).size(size)
    }

    pub fn view_action_button<'a, S: Into<String>, T: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        surface: ButtonSurface,
        explanation: T,
    ) -> Element<'a, Message> {
        let label = label.into();
        let explanation = explanation.into();
        let mut button = Button::new(Text::new(label).size(size))
            .padding([8, 12])
            .style(action_button_style(surface));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        tooltip(
            button,
            Container::new(Text::new(explanation).size(12))
                .padding([8, 10])
                .style(tooltip_container_style),
            tooltip::Position::Top,
        )
        .gap(8)
        .into()
    }

    pub fn view_symbol_action_button<'a, S: Into<String>, T: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        surface: ButtonSurface,
        explanation: T,
    ) -> Element<'a, Message> {
        let explanation = explanation.into();
        let mut button = Button::new(self.symbol_text(label, size))
            .padding([8, 12])
            .style(action_button_style(surface));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        tooltip(
            button,
            Container::new(Text::new(explanation).size(12))
                .padding([8, 10])
                .style(tooltip_container_style),
            tooltip::Position::Top,
        )
        .gap(8)
        .into()
    }

    pub fn view_action_button_with_width<'a, S: Into<String>, T: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        surface: ButtonSurface,
        explanation: T,
        width: Length,
    ) -> Element<'a, Message> {
        let label = label.into();
        let explanation = explanation.into();
        let mut button = Button::new(
            Text::new(label)
                .size(size)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .padding([8, 12])
        .width(width)
        .style(action_button_style(surface));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        tooltip(
            button,
            Container::new(Text::new(explanation).size(12))
                .padding([8, 10])
                .style(tooltip_container_style),
            tooltip::Position::Top,
        )
        .gap(8)
        .into()
    }

    pub fn view_symbol_action_button_with_width<'a, S: Into<String>, T: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        surface: ButtonSurface,
        explanation: T,
        width: Length,
    ) -> Element<'a, Message> {
        let explanation = explanation.into();
        let mut button = Button::new(
            self.symbol_text(label, size)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .padding([8, 12])
        .width(width)
        .style(action_button_style(surface));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        tooltip(
            button,
            Container::new(Text::new(explanation).size(12))
                .padding([8, 10])
                .style(tooltip_container_style),
            tooltip::Position::Top,
        )
        .gap(8)
        .into()
    }

    pub fn view_plain_button<'a, S: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        text_color: iced::Color,
    ) -> Element<'a, Message> {
        let mut button = Button::new(Text::new(label.into()).size(size))
            .padding([8, 10])
            .style(inline_button_style(text_color));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        button.into()
    }

    pub fn view_symbol_plain_button<'a, S: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        text_color: iced::Color,
    ) -> Element<'a, Message> {
        let mut button = Button::new(self.symbol_text(label.into(), size))
            .padding([8, 10])
            .style(inline_button_style(text_color));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        button.into()
    }

    pub fn view_plain_button_fill<'a, S: Into<String>>(
        &self,
        label: S,
        size: u16,
        message: Option<Message>,
        surface: ButtonSurface,
    ) -> Element<'a, Message> {
        let mut button = Button::new(Text::new(label.into()).size(size).width(Length::Fill))
            .padding([8, 12])
            .width(Length::Fill)
            .style(action_button_style(surface));

        if let Some(message) = message {
            button = button.on_press(message);
        }

        button.into()
    }
}
