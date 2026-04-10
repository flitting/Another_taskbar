use iced::widget::{button, pick_list, scrollable, text_editor, text_input};
use iced::{Background, Border, Color, Vector};
use std::rc::Rc;
use std::sync::{OnceLock, RwLock};

use crate::symbols::{
    SYMBOL_ARCHIVED, SYMBOL_BLOCKED, SYMBOL_COMPLETED, SYMBOL_IN_PROGRESS, SYMBOL_TODO,
};
use crate::tasks::TaskState;

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub name: String,
    pub primary_bg: Color,
    pub secondary_bg: Color,
    pub tertiary_bg: Color,
    pub pinned_bg: Color,
    pub accent_color: Color,
    pub highlight_bg: Color,
    pub selection_bg: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub menu_bg: Color,
    pub tooltip_bg: Color,
    pub tag_bg: Color,
    pub tag_active_bg: Color,
    pub input_bg: Color,
    pub todo_color: Color,
    pub in_progress_color: Color,
    pub blocked_color: Color,
    pub completed_color: Color,
    pub archived_color: Color,
    pub importance_high_stripe: Color,
    pub importance_low_stripe: Color,
    pub urgency_high_stripe: Color,
    pub urgency_low_stripe: Color,
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            name: "Dark".to_string(),
            primary_bg: Color::from_rgb(0.10, 0.10, 0.15),
            secondary_bg: Color::from_rgb(0.15, 0.15, 0.22),
            tertiary_bg: Color::from_rgb(0.20, 0.20, 0.28),
            pinned_bg: Color::from_rgb(0.24, 0.18, 0.10),
            accent_color: Color::from_rgb(0.29, 0.53, 0.81),
            highlight_bg: Color::from_rgb(0.35, 0.25, 0.55),
            selection_bg: Color::from_rgb(0.22, 0.46, 0.72),
            text_primary: Color::from_rgb(0.95, 0.95, 0.98),
            text_secondary: Color::from_rgb(0.70, 0.70, 0.78),
            text_muted: Color::from_rgb(0.50, 0.50, 0.58),
            menu_bg: Color::from_rgb(0.28, 0.28, 0.36),
            tooltip_bg: Color::from_rgb(0.09, 0.09, 0.13),
            tag_bg: Color::from_rgb(0.24, 0.24, 0.28),
            tag_active_bg: Color::from_rgb(0.44, 0.44, 0.52),
            input_bg: Color::from_rgb(0.12, 0.12, 0.18),
            todo_color: Color::from_rgb(0.45, 0.53, 0.73),
            in_progress_color: Color::from_rgb(0.89, 0.68, 0.28),
            blocked_color: Color::from_rgb(0.83, 0.45, 0.45),
            completed_color: Color::from_rgb(0.45, 0.78, 0.53),
            archived_color: Color::from_rgb(0.50, 0.50, 0.58),
            importance_high_stripe: Color::from_rgb(0.86, 0.28, 0.28),
            importance_low_stripe: Color::from_rgb(0.28, 0.74, 0.38),
            urgency_high_stripe: Color::from_rgb(0.92, 0.56, 0.18),
            urgency_low_stripe: Color::from_rgb(0.42, 0.82, 0.98),
        }
    }
}

static ACTIVE_THEME: OnceLock<RwLock<ThemePalette>> = OnceLock::new();

fn theme_store() -> &'static RwLock<ThemePalette> {
    ACTIVE_THEME.get_or_init(|| RwLock::new(ThemePalette::default()))
}

pub fn current_theme_palette() -> ThemePalette {
    theme_store()
        .read()
        .map(|palette| palette.clone())
        .unwrap_or_default()
}

pub fn apply_theme_palette(palette: ThemePalette) {
    if let Ok(mut active) = theme_store().write() {
        *active = palette;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSurface {
    Highlight,
    Tertiary,
    Meta,
    Tag,
    TagActive,
}

pub const HOVER_OVERLAY: Color = Color::from_rgba(0.95, 0.95, 0.98, 0.10);
pub const PRESSED_OVERLAY: Color = Color::from_rgba(0.95, 0.95, 0.98, 0.18);
const POPUP_BORDER_WIDTH: f32 = 1.0;
const POPUP_RADIUS: f32 = 24.0;

pub fn get_state_color(state: &TaskState) -> Color {
    let palette = current_theme_palette();
    match state {
        TaskState::Todo => palette.todo_color,
        TaskState::InProgress => palette.in_progress_color,
        TaskState::Blocked => palette.blocked_color,
        TaskState::Completed => palette.completed_color,
        TaskState::Archived => palette.archived_color,
    }
}

pub fn task_state_icon(state: &TaskState) -> &'static str {
    match state {
        TaskState::Todo => SYMBOL_TODO,
        TaskState::InProgress => SYMBOL_IN_PROGRESS,
        TaskState::Blocked => SYMBOL_BLOCKED,
        TaskState::Completed => SYMBOL_COMPLETED,
        TaskState::Archived => SYMBOL_ARCHIVED,
    }
}

pub fn all_task_states() -> Vec<TaskState> {
    vec![
        TaskState::Todo,
        TaskState::InProgress,
        TaskState::Blocked,
        TaskState::Completed,
        TaskState::Archived,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStateOption {
    pub state: TaskState,
    pub label: String,
}

impl std::fmt::Display for TaskStateOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub fn task_state_option(state: &TaskState) -> TaskStateOption {
    TaskStateOption {
        state: state.clone(),
        label: task_state_label(state),
    }
}

pub fn all_task_state_options() -> Vec<TaskStateOption> {
    all_task_states()
        .into_iter()
        .map(|state| task_state_option(&state))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStateIconOption {
    pub state: TaskState,
    pub label: String,
}

impl std::fmt::Display for TaskStateIconOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub fn task_state_icon_option(state: &TaskState) -> TaskStateIconOption {
    TaskStateIconOption {
        state: state.clone(),
        label: match state {
            TaskState::Todo => task_state_icon(state).to_string(),
            TaskState::InProgress
            | TaskState::Blocked
            | TaskState::Completed
            | TaskState::Archived => format!(" {}", task_state_icon(state)),
        },
    }
}

pub fn all_task_state_icon_options() -> Vec<TaskStateIconOption> {
    all_task_states()
        .into_iter()
        .map(|state| task_state_icon_option(&state))
        .collect()
}

pub fn task_state_label(state: &TaskState) -> String {
    format!("{} {}", task_state_icon(state), state)
}

pub fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
}

fn container_appearance(background: Color) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(background)),
        text_color: Some(palette.text_primary),
        ..Default::default()
    }
}

fn popup_appearance(background: Color) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(background)),
        text_color: Some(palette.text_primary),
        border: Border {
            radius: POPUP_RADIUS.into(),
            width: POPUP_BORDER_WIDTH,
            color: mix_colors(palette.accent_color, palette.menu_bg),
        },
        ..Default::default()
    }
}

fn popup_inner_appearance(background: Color) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(background)),
        text_color: Some(palette.text_primary),
        border: Border {
            radius: (POPUP_RADIUS - POPUP_BORDER_WIDTH).max(0.0).into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn container_primary_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    container_appearance(current_theme_palette().primary_bg)
}

pub fn container_secondary_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    container_appearance(current_theme_palette().secondary_bg)
}

pub fn container_tertiary_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    container_appearance(current_theme_palette().tertiary_bg)
}

pub fn container_input_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(palette.input_bg)),
        text_color: Some(palette.text_primary),
        border: Border {
            radius: 10.0.into(),
            width: 1.0,
            color: mix_colors(palette.input_bg, HOVER_OVERLAY),
        },
        ..Default::default()
    }
}

pub fn container_pinned_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(palette.pinned_bg)),
        text_color: Some(palette.text_primary),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: palette.highlight_bg,
        },
        ..Default::default()
    }
}

pub fn container_highlight_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    container_appearance(current_theme_palette().highlight_bg)
}

pub fn text_primary_container_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(current_theme_palette().text_primary),
        ..Default::default()
    }
}

pub fn modal_backdrop_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        background: Some(Background::Color(current_theme_palette().secondary_bg)),
        ..Default::default()
    }
}

fn mix_colors(base: Color, overlay: Color) -> Color {
    Color {
        r: (base.r + overlay.r) * 0.5,
        g: (base.g + overlay.g) * 0.5,
        b: (base.b + overlay.b) * 0.5,
        a: 1.0,
    }
}

fn surface_color(surface: ButtonSurface) -> Color {
    let palette = current_theme_palette();
    match surface {
        ButtonSurface::Highlight => palette.highlight_bg,
        ButtonSurface::Tertiary => palette.tertiary_bg,
        ButtonSurface::Meta => palette.menu_bg,
        ButtonSurface::Tag => palette.tag_bg,
        ButtonSurface::TagActive => palette.tag_active_bg,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionButtonStyle {
    surface: ButtonSurface,
}

impl button::StyleSheet for ActionButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let palette = current_theme_palette();
        let border_color = match self.surface {
            ButtonSurface::Meta => mix_colors(palette.accent_color, palette.menu_bg),
            _ => Color::TRANSPARENT,
        };
        let border_width = match self.surface {
            ButtonSurface::Meta => 1.0,
            _ => 0.0,
        };
        button::Appearance {
            background: Some(Background::Color(surface_color(self.surface))),
            text_color: palette.text_primary,
            border: Border {
                radius: 8.0.into(),
                width: border_width,
                color: border_color,
            },
            shadow_offset: Vector::default(),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.background = Some(Background::Color(mix_colors(
            surface_color(self.surface),
            HOVER_OVERLAY,
        )));
        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.background = Some(Background::Color(mix_colors(
            surface_color(self.surface),
            PRESSED_OVERLAY,
        )));
        appearance
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.text_color = current_theme_palette().text_muted;
        appearance
    }
}

pub fn action_button_style(surface: ButtonSurface) -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(ActionButtonStyle { surface }))
}

#[derive(Debug, Clone, Copy)]
pub struct InlineButtonStyle {
    text_color: Color,
}

impl button::StyleSheet for InlineButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            text_color: self.text_color,
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow_offset: Vector::default(),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.text_color = mix_colors(self.text_color, current_theme_palette().text_primary);
        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.text_color = mix_colors(self.text_color, PRESSED_OVERLAY);
        appearance
    }
}

pub fn inline_button_style(text_color: Color) -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(InlineButtonStyle { text_color }))
}

pub fn container_menu_bg_light_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    popup_appearance(current_theme_palette().menu_bg)
}

pub fn popup_window_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    popup_appearance(current_theme_palette().secondary_bg)
}

pub fn popup_window_inner_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    popup_inner_appearance(current_theme_palette().secondary_bg)
}

pub fn tooltip_container_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
    let palette = current_theme_palette();
    iced::widget::container::Appearance {
        background: Some(Background::Color(palette.tooltip_bg)),
        text_color: Some(palette.text_primary),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: palette.text_secondary,
        },
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DarkTextInputStyle;

impl text_input::StyleSheet for DarkTextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = current_theme_palette();
        text_input::Appearance {
            background: Background::Color(palette.input_bg),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: palette.menu_bg,
            },
            icon_color: palette.text_muted,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = current_theme_palette().accent_color;
        appearance
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_muted
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_primary
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_muted
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().selection_bg
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}

pub fn dark_text_input_style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(DarkTextInputStyle))
}

#[derive(Debug, Clone, Copy)]
pub struct DarkPickListStyle;

impl pick_list::StyleSheet for DarkPickListStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        let palette = current_theme_palette();
        pick_list::Appearance {
            text_color: palette.text_primary,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_muted,
            background: Background::Color(palette.input_bg),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: palette.menu_bg,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = current_theme_palette().highlight_bg;
        appearance
    }
}

pub fn dark_pick_list_style() -> iced::theme::PickList {
    iced::theme::PickList::Custom(Rc::new(DarkPickListStyle), Rc::new(DarkMenuStyle))
}

#[derive(Debug, Clone, Copy)]
pub struct CompactDarkPickListStyle;

impl pick_list::StyleSheet for CompactDarkPickListStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        let palette = current_theme_palette();
        pick_list::Appearance {
            text_color: palette.text_primary,
            placeholder_color: palette.text_muted,
            handle_color: Color::TRANSPARENT,
            background: Background::Color(palette.input_bg),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: palette.menu_bg,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = current_theme_palette().highlight_bg;
        appearance
    }
}

pub fn compact_dark_pick_list_style() -> iced::theme::PickList {
    iced::theme::PickList::Custom(Rc::new(CompactDarkPickListStyle), Rc::new(DarkMenuStyle))
}

#[derive(Debug, Clone, Copy)]
pub struct DarkScrollableStyle;

impl scrollable::StyleSheet for DarkScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        let palette = current_theme_palette();
        scrollable::Appearance {
            container: iced::widget::container::Appearance::default(),
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(palette.secondary_bg)),
                border: Border {
                    radius: 8.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                scroller: scrollable::Scroller {
                    color: palette.menu_bg,
                    border: Border {
                        radius: 8.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
            },
            gap: Some(Background::Color(palette.secondary_bg)),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        let mut appearance = self.active(style);
        if is_mouse_over_scrollbar {
            appearance.scrollbar.scroller.color =
                mix_colors(current_theme_palette().menu_bg, HOVER_OVERLAY);
        }
        appearance
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Appearance {
        let mut appearance = self.active(style);
        appearance.scrollbar.scroller.color =
            mix_colors(current_theme_palette().menu_bg, PRESSED_OVERLAY);
        appearance
    }
}

pub fn dark_scrollable_style() -> iced::theme::Scrollable {
    iced::theme::Scrollable::Custom(Box::new(DarkScrollableStyle))
}

#[derive(Debug, Clone, Copy)]
pub struct DarkMenuStyle;

impl iced::overlay::menu::StyleSheet for DarkMenuStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::overlay::menu::Appearance {
        let palette = current_theme_palette();
        iced::overlay::menu::Appearance {
            text_color: palette.text_primary,
            background: Background::Color(palette.input_bg),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: palette.menu_bg,
            },
            selected_text_color: palette.text_primary,
            selected_background: Background::Color(palette.highlight_bg),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DetailTextEditorStyle;

impl text_editor::StyleSheet for DetailTextEditorStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_editor::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_muted
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_primary
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().text_muted
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        current_theme_palette().selection_bg
    }

    fn disabled(&self, style: &Self::Style) -> text_editor::Appearance {
        self.active(style)
    }
}

pub fn detail_text_editor_style() -> iced::theme::TextEditor {
    iced::theme::TextEditor::Custom(Box::new(DetailTextEditorStyle))
}
