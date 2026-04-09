use std::collections::HashSet;

use chrono::Utc;
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};

use crate::files::load_taskbar;
use crate::tasks::{Task, TaskManager, TaskState};

// Modern dark theme colors
const PRIMARY_BG: Color = Color::from_rgb(0.10, 0.10, 0.15);
const SECONDARY_BG: Color = Color::from_rgb(0.15, 0.15, 0.22);
const TERTIARY_BG: Color = Color::from_rgb(0.20, 0.20, 0.28);
const ACCENT_COLOR: Color = Color::from_rgb(0.29, 0.53, 0.81);
const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.98);
const TEXT_SECONDARY: Color = Color::from_rgb(0.70, 0.70, 0.78);
const TEXT_MUTED: Color = Color::from_rgb(0.50, 0.50, 0.58);

// State colors
const TODO_COLOR: Color = Color::from_rgb(0.45, 0.53, 0.73);
const IN_PROGRESS_COLOR: Color = Color::from_rgb(0.89, 0.68, 0.28);
const BLOCKED_COLOR: Color = Color::from_rgb(0.83, 0.45, 0.45);
const COMPLETED_COLOR: Color = Color::from_rgb(0.45, 0.78, 0.53);
const ARCHIVED_COLOR: Color = Color::from_rgb(0.50, 0.50, 0.58);

#[derive(Debug, Clone)]
pub enum Message {
    ToggleCollapse(u32),
    ToggleDetail(u32),
    ToggleStateMenu(u32),
    SelectState(u32, TaskState),
    CloseDetail,
}

pub struct Gui {
    manager: TaskManager,
    collapsed: HashSet<u32>,
    detail_task: Option<u32>,
    state_menu_for: Option<u32>,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        let manager = match load_taskbar("taskbar.json") {
            Ok(m) => m,
            Err(_) => {
                let mut m = TaskManager::new();

                let mut t1 = Task::empty_task(0);
                t1.name = "Buy groceries".to_string();
                t1.description = "Milk, Eggs, Bread".to_string();

                let mut t2 = Task::empty_task(0);
                t2.name = "Build prototype".to_string();
                t2.description = "Create iced GUI prototype".to_string();

                let mut st1 = Task::empty_task(0);
                st1.name = "Write UI".to_string();
                st1.description = "Implement list and details".to_string();

                let mut st2 = Task::empty_task(0);
                st2.name = "Write tests".to_string();
                st2.description = "Add unit tests".to_string();

                t2.add_subtask(st1);
                t2.add_subtask(st2);

                m.add_task(0, t1).ok();
                m.add_task(0, t2).ok();

                m
            }
        };

        Gui {
            manager,
            collapsed: HashSet::new(),
            detail_task: None,
            state_menu_for: None,
        }
    }

    fn title(&self) -> String {
        "Task Manager".into()
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
                if self.detail_task == Some(id) {
                    self.detail_task = None;
                } else {
                    self.detail_task = Some(id);
                }
                self.state_menu_for = None;
            }
            Message::ToggleStateMenu(id) => {
                if self.state_menu_for == Some(id) {
                    self.state_menu_for = None;
                } else {
                    self.state_menu_for = Some(id);
                }
            }
            Message::SelectState(id, new_state) => {
                if let Some(t) = self.manager.root.search_by_id(id) {
                    t.state = new_state;
                    t.times.updated_at = Utc::now();
                }
                self.state_menu_for = None;
            }
            Message::CloseDetail => {
                self.detail_task = None;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut main_row = Row::new()
            .spacing(16)
            .align_items(Alignment::Start)
            .push(self.view_task_list());

        if let Some(task_id) = self.detail_task {
            if let Some(task) = self.manager.root.search_by_id_ref(task_id) {
                main_row = main_row.push(self.view_detail(task));
            }
        }

        Container::new(main_row.padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Gui {
    fn view_task_list(&self) -> Element<'_, Message> {
        let header = Text::new("📋 Tasks").size(32);

        let mut task_column = Column::new().spacing(12).push(header);

        for task in &self.manager.root.subtasks {
            task_column = task_column.push(self.render_task_recursive(task, 0));
        }

        let scrollable_content = Scrollable::new(task_column)
            .height(Length::Fill)
            .width(Length::FillPortion(2));

        Container::new(scrollable_content)
            .padding(16)
            .height(Length::Fill)
            .into()
    }

    fn render_task_recursive<'a>(&self, task: &'a Task, depth: usize) -> Element<'a, Message> {
        let indent = depth * 24;
        let state_color = get_state_color(&task.state);

        let mut task_row = Row::new().spacing(12).align_items(Alignment::Center);

        if indent > 0 {
            task_row = task_row.push(Space::with_width(Length::Fixed(indent as f32)));
        }

        // Collapse/expand button
        if !task.subtasks.is_empty() {
            let symbol = if self.collapsed.contains(&task.id) {
                "▶"
            } else {
                "▼"
            };
            task_row = task_row.push(
                Button::new(Text::new(symbol).size(16))
                    .padding(6)
                    .on_press(Message::ToggleCollapse(task.id)),
            );
        } else {
            task_row = task_row.push(Space::with_width(Length::Fixed(28.0)));
        }

        // State indicator
        let state_icon = task_state_icon(&task.state);
        task_row = task_row.push(Container::new(Text::new(state_icon).size(18)).padding(8));

        // Task name button
        task_row = task_row.push(
            Button::new(Text::new(&task.name).size(16).width(Length::Fill))
                .padding([8, 12])
                .on_press(Message::ToggleDetail(task.id)),
        );

        // State menu button
        task_row = task_row.push(
            Button::new(Text::new("⚙").size(14))
                .padding(6)
                .on_press(Message::ToggleStateMenu(task.id)),
        );

        let mut container_col = Column::new().spacing(8).push(task_row);

        // State menu dropdown
        if self.state_menu_for == Some(task.id) {
            let mut menu = Column::new().spacing(4).padding(12);

            for s in all_task_states() {
                let label = format!("{} {}", task_state_icon(&s), s);
                menu = menu.push(
                    Button::new(Text::new(label).size(13))
                        .padding([6, 10])
                        .width(Length::Fill)
                        .on_press(Message::SelectState(task.id, s.clone())),
                );
            }

            container_col = container_col.push(Container::new(menu).padding(0).width(Length::Fill));
        }

        // Recursive subtasks
        if !task.subtasks.is_empty() && !self.collapsed.contains(&task.id) {
            for sub in &task.subtasks {
                container_col = container_col.push(self.render_task_recursive(sub, depth + 1));
            }
        }

        Container::new(container_col)
            .padding(8)
            .width(Length::Fill)
            .into()
    }

    fn view_detail<'a>(&self, task: &'a Task) -> Element<'a, Message> {
        let state_color = get_state_color(&task.state);

        let header = Column::new()
            .spacing(8)
            .push(Text::new(&task.name).size(28))
            .push(
                Row::new()
                    .spacing(12)
                    .align_items(Alignment::Center)
                    .push(
                        Container::new(Text::new(task_state_icon(&task.state)).size(20)).padding(8),
                    )
                    .push(Text::new(task.state.to_string()).size(16)),
            );

        let mut detail_col = Column::new().spacing(16).push(header).push(
            Column::new()
                .spacing(6)
                .push(Text::new("Description").size(14))
                .push(
                    Container::new(Text::new(&task.description).size(14))
                        .padding(12)
                        .width(Length::Fill),
                ),
        );

        if !task.tags.is_empty() {
            let mut tags_row = Row::new().spacing(8);
            for tag in &task.tags {
                tags_row = tags_row.push(Container::new(Text::new(tag).size(12)).padding([4, 10]));
            }

            detail_col = detail_col.push(
                Column::new()
                    .spacing(6)
                    .push(Text::new("Tags").size(14))
                    .push(tags_row),
            );
        }

        detail_col = detail_col
            .push(
                Column::new()
                    .spacing(6)
                    .push(Text::new("Dates").size(14))
                    .push(
                        Text::new(format!("Created: {}", format_date(task.times.created_at)))
                            .size(12),
                    )
                    .push(
                        Text::new(format!("Updated: {}", format_date(task.times.updated_at)))
                            .size(12),
                    ),
            )
            .push(
                Button::new(Text::new("✕ Close").size(14))
                    .padding([10, 16])
                    .width(Length::Fill)
                    .on_press(Message::CloseDetail),
            );

        Container::new(
            Scrollable::new(Container::new(detail_col).padding(16).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
        )
        .padding(16)
        .height(Length::Fill)
        .into()
    }
}

fn get_state_color(state: &TaskState) -> Color {
    match state {
        TaskState::Todo => TODO_COLOR,
        TaskState::InProgress => IN_PROGRESS_COLOR,
        TaskState::Blocked => BLOCKED_COLOR,
        TaskState::Completed => COMPLETED_COLOR,
        TaskState::Archived => ARCHIVED_COLOR,
    }
}

fn task_state_icon(state: &TaskState) -> &'static str {
    match state {
        TaskState::Todo => "◯",
        TaskState::InProgress => "◐",
        TaskState::Blocked => "⚠",
        TaskState::Completed => "✓",
        TaskState::Archived => "📦",
    }
}

fn all_task_states() -> Vec<TaskState> {
    vec![
        TaskState::Todo,
        TaskState::InProgress,
        TaskState::Blocked,
        TaskState::Completed,
        TaskState::Archived,
    ]
}

fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
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
    Gui::run(Settings::default()).map_err(|e| e.to_string())
}
