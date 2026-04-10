use iced::widget::{Column, Container, Row, Space, Text};
use iced::{Alignment, Element, Length};

use crate::gui::theme::{container_menu_bg_light_style, modal_backdrop_style, ButtonSurface};
use crate::symbols::SYMBOL_CLOSE;
use crate::tasks::{ImportanceFilter, PinnedFilter, StateFilter, TaskState, UrgencyFilter};

use super::super::app::{Gui, Message};

const FILTER_TAG_ROW_WRAP_WIDTH: f32 = 380.0;
const FILTER_TAG_BUTTON_BASE_WIDTH: f32 = 28.0;
const FILTER_TAG_BUTTON_CHAR_WIDTH: f32 = 8.5;
const FILTER_TAG_ROW_MAX_ITEMS: usize = 5;

impl Gui {
    pub fn view_filter_modal(&self) -> Element<'_, Message> {
        let mut content = Column::new()
            .spacing(14)
            .push(
                Row::new()
                    .spacing(8)
                    .align_items(Alignment::Center)
                    .push(Text::new("Filter").size(22))
                    .push(Space::with_width(Length::Fill))
                    .push(self.view_action_button(
                        SYMBOL_CLOSE,
                        14,
                        Some(Message::CancelFilterSelection),
                        ButtonSurface::Tertiary,
                        "Close the filter panel without applying draft changes.",
                    )),
            )
            .push(Text::new("Select tags, importance, and urgency to show matching tasks. Parent tasks stay visible if any nested subtask matches.").size(12))
            .push(Text::new("Importance").size(14))
            .push(self.view_importance_filter_row())
            .push(Text::new("Urgency").size(14))
            .push(self.view_urgency_filter_row())
            .push(Text::new("State").size(14))
            .push(self.view_state_filter_rows())
            .push(Text::new("Pinned").size(14))
            .push(self.view_pinned_filter_row())
            .push(Text::new("Tags").size(14));

        let mut tag_items = Vec::new();
        for tag in &self.manager.available_tags {
            let selected = self.draft_filter_tags.iter().any(|value| value == tag);
            tag_items.push((
                self.view_action_button(
                    tag.clone(),
                    13,
                    Some(Message::ToggleDraftFilterTag(tag.clone())),
                    if selected {
                        ButtonSurface::Highlight
                    } else {
                        ButtonSurface::Tertiary
                    },
                    format!("Toggle the '{tag}' filter."),
                ),
                Self::estimated_filter_tag_button_width(tag),
            ));
        }

        content = content
            .push(Self::view_filter_tag_rows(tag_items))
            .push(
            Row::new()
                .spacing(8)
                .push(self.view_action_button(
                    "Clear All",
                    14,
                    Some(Message::ClearDraftFilterTags),
                    ButtonSurface::Tertiary,
                    "Clear all selected filters.",
                ))
                .push(self.view_action_button(
                    "Confirm",
                    14,
                    Some(Message::ApplyFilterSelection),
                    ButtonSurface::Highlight,
                    "Apply the selected tag filters.",
                ))
                .push(self.view_action_button(
                    "Cancel",
                    14,
                    Some(Message::CancelFilterSelection),
                    ButtonSurface::Tertiary,
                    "Close without changing the current filters.",
                )),
        );

        Container::new(content)
            .padding(18)
            .width(Length::Fixed(420.0))
            .style(container_menu_bg_light_style)
            .into()
    }

    pub fn view_filter_overlay(&self) -> Element<'_, Message> {
        Container::new(
            Column::new()
                .push(Space::with_height(Length::FillPortion(1)))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(Space::with_width(Length::Fill))
                        .push(self.view_filter_modal())
                        .push(Space::with_width(Length::Fill)),
                )
                .push(Space::with_height(Length::FillPortion(2))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(modal_backdrop_style)
        .into()
    }

    fn view_importance_filter_row(&self) -> Element<'_, Message> {
        Row::new()
            .spacing(8)
            .push(self.view_filter_option_button(
                "High",
                self.draft_importance_filter == ImportanceFilter::High,
                Message::SelectDraftImportanceFilter(ImportanceFilter::High),
                "Show tasks with high importance.",
            ))
            .push(self.view_filter_option_button(
                "Low",
                self.draft_importance_filter == ImportanceFilter::Low,
                Message::SelectDraftImportanceFilter(ImportanceFilter::Low),
                "Show tasks with low importance.",
            ))
            .push(self.view_filter_option_button(
                "Neither",
                self.draft_importance_filter == ImportanceFilter::Neither,
                Message::SelectDraftImportanceFilter(ImportanceFilter::Neither),
                "Show tasks without an importance value.",
            ))
            .into()
    }

    fn view_urgency_filter_row(&self) -> Element<'_, Message> {
        Row::new()
            .spacing(8)
            .push(self.view_filter_option_button(
                "High",
                self.draft_urgency_filter == UrgencyFilter::High,
                Message::SelectDraftUrgencyFilter(UrgencyFilter::High),
                "Show tasks with high urgency.",
            ))
            .push(self.view_filter_option_button(
                "Low",
                self.draft_urgency_filter == UrgencyFilter::Low,
                Message::SelectDraftUrgencyFilter(UrgencyFilter::Low),
                "Show tasks with low urgency.",
            ))
            .push(self.view_filter_option_button(
                "Neither",
                self.draft_urgency_filter == UrgencyFilter::Neither,
                Message::SelectDraftUrgencyFilter(UrgencyFilter::Neither),
                "Show tasks without an urgency value.",
            ))
            .into()
    }

    fn view_pinned_filter_row(&self) -> Element<'_, Message> {
        Row::new()
            .spacing(8)
            .push(self.view_filter_option_button(
                "Pinned",
                self.draft_pinned_filter == PinnedFilter::Pinned,
                Message::SelectDraftPinnedFilter(PinnedFilter::Pinned),
                "Show pinned tasks.",
            ))
            .push(self.view_filter_option_button(
                "Unpinned",
                self.draft_pinned_filter == PinnedFilter::Unpinned,
                Message::SelectDraftPinnedFilter(PinnedFilter::Unpinned),
                "Show unpinned tasks.",
            ))
            .into()
    }

    fn view_state_filter_rows(&self) -> Element<'_, Message> {
        Column::new()
            .spacing(8)
            .push(
                Row::new()
                    .spacing(8)
                    .push(self.view_state_filter_button(TaskState::Todo, "Todo"))
                    .push(self.view_state_filter_button(TaskState::InProgress, "InProgress"))
                    .push(self.view_state_filter_button(TaskState::Blocked, "Blocked")),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .push(self.view_state_filter_button(TaskState::Completed, "Completed"))
                    .push(self.view_state_filter_button(TaskState::Archived, "Archived"))
                    .push(self.view_filter_option_button(
                        "None",
                        self.draft_state_filter == StateFilter::None,
                        Message::SelectDraftStateFilter(StateFilter::None),
                        "Show no tasks for the special 'None' state filter.",
                    )),
            )
            .into()
    }

    fn view_state_filter_button(
        &self,
        state: TaskState,
        label: &'static str,
    ) -> Element<'_, Message> {
        let filter = match state {
            TaskState::Todo => StateFilter::Todo,
            TaskState::InProgress => StateFilter::InProgress,
            TaskState::Blocked => StateFilter::Blocked,
            TaskState::Completed => StateFilter::Completed,
            TaskState::Archived => StateFilter::Archived,
        };

        self.view_filter_option_button(
            label,
            self.draft_state_filter == filter,
            Message::SelectDraftStateFilter(filter),
            "Show tasks in this state.",
        )
    }

    fn view_filter_option_button(
        &self,
        label: &'static str,
        selected: bool,
        message: Message,
        tooltip: &'static str,
    ) -> Element<'_, Message> {
        self.view_action_button(
            label,
            13,
            Some(message),
            if selected {
                ButtonSurface::Highlight
            } else {
                ButtonSurface::Tertiary
            },
            tooltip,
        )
    }

    fn estimated_filter_tag_button_width(tag: &str) -> f32 {
        FILTER_TAG_BUTTON_BASE_WIDTH + tag.chars().count() as f32 * FILTER_TAG_BUTTON_CHAR_WIDTH
    }

    fn view_filter_tag_rows<'a>(items: Vec<(Element<'a, Message>, f32)>) -> Element<'a, Message> {
        let mut rows: Vec<Vec<Element<'a, Message>>> = Vec::new();
        let mut current_row: Vec<Element<'a, Message>> = Vec::new();
        let mut current_width = 0.0;

        for (item, width) in items {
            let spacing = if current_row.is_empty() { 0.0 } else { 8.0 };

            if !current_row.is_empty()
                && (current_width + spacing + width > FILTER_TAG_ROW_WRAP_WIDTH
                    || current_row.len() >= FILTER_TAG_ROW_MAX_ITEMS)
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
