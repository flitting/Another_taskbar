#![allow(dead_code)]

use another_taskbar::tasks::{
    CustomRecurrence, RecurrenceEnd, RecurrenceFrequency, RecurrenceSetting, RecurrenceUnit, Task,
    TaskDraft, TaskManager, TaskState,
};
use chrono::{Duration, Utc};

pub fn draft_named(name: &str) -> TaskDraft {
    TaskDraft {
        name: name.to_string(),
        description: String::new(),
        state: TaskState::Todo,
        urgency: None,
        importance: None,
        tags: Vec::new(),
        pinned: false,
        due_date: None,
        completed_at: None,
        recurrence: None,
    }
}

pub fn create_root_task(manager: &mut TaskManager, name: &str) -> u32 {
    manager
        .create_task_from_draft(0, draft_named(name))
        .expect("task should be created")
}

pub fn root_task<'a>(manager: &'a TaskManager, id: u32) -> &'a Task {
    manager
        .root
        .subtasks
        .iter()
        .find(|task| task.id == id)
        .expect("root task should exist")
}

pub fn daily_recurrence_in_past() -> RecurrenceSetting {
    RecurrenceSetting {
        frequency: RecurrenceFrequency::Daily,
        due_hour: 9,
        due_minute: 30,
        custom: None,
        occurrences_done: 0,
    }
}

pub fn custom_weekly_after(count: u32) -> RecurrenceSetting {
    RecurrenceSetting {
        frequency: RecurrenceFrequency::Custom,
        due_hour: 8,
        due_minute: 15,
        custom: Some(CustomRecurrence {
            every: 1,
            unit: RecurrenceUnit::Week,
            end: RecurrenceEnd::AfterOccurrences(count),
        }),
        occurrences_done: 0,
    }
}

pub fn due_days_ago(days: i64) -> chrono::DateTime<Utc> {
    Utc::now() - Duration::days(days)
}
