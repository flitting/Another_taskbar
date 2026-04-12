mod common;

use another_taskbar::tasks::{TaskState, TaskSortMode};

use common::{create_root_task, custom_weekly_after, daily_recurrence_in_past, due_days_ago};

#[test]
fn recurring_task_resets_to_todo_and_moves_due_date_forward() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let task_id = create_root_task(&mut manager, "Recurring");
    let task = manager.root.search_by_id(task_id).unwrap();
    task.state = TaskState::Completed;
    task.times.due_date = Some(due_days_ago(2));
    task.recurrence = Some(daily_recurrence_in_past());

    manager.apply_recurring_updates();

    let task = manager.root.search_by_id_ref(task_id).unwrap();
    assert_eq!(task.state, TaskState::Todo);
    assert!(task.times.due_date.unwrap() > due_days_ago(0));
}

#[test]
fn blocked_recurring_task_keeps_blocked_state() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let task_id = create_root_task(&mut manager, "Blocked recurring");
    let task = manager.root.search_by_id(task_id).unwrap();
    task.state = TaskState::Blocked;
    task.times.due_date = Some(due_days_ago(3));
    task.recurrence = Some(daily_recurrence_in_past());

    manager.apply_recurring_updates();

    let task = manager.root.search_by_id_ref(task_id).unwrap();
    assert_eq!(task.state, TaskState::Blocked);
}

#[test]
fn custom_recurrence_tracks_occurrences() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let task_id = create_root_task(&mut manager, "Custom recurring");
    let task = manager.root.search_by_id(task_id).unwrap();
    task.times.due_date = Some(due_days_ago(14));
    task.recurrence = Some(custom_weekly_after(3));

    manager.apply_recurring_updates();
    manager.sort_for_mode(&TaskSortMode::UpdateFirst);

    let task = manager.root.search_by_id_ref(task_id).unwrap();
    assert!(task.recurrence.as_ref().unwrap().occurrences_done >= 1);
    assert!(task.times.due_date.is_some());
}
