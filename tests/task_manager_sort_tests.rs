mod common;

use another_taskbar::tasks::TaskSortMode;

use common::{create_root_task, root_task};

#[test]
fn custom_sort_keeps_manual_reorder() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let first = create_root_task(&mut manager, "Alpha");
    let second = create_root_task(&mut manager, "Beta");
    let third = create_root_task(&mut manager, "Gamma");

    manager.move_task_before(third, first).unwrap();
    manager.sort_for_mode(&TaskSortMode::Custom);

    let ordered: Vec<u32> = manager.root.subtasks.iter().map(|task| task.id).collect();
    assert_eq!(ordered, vec![third, first, second]);
}

#[test]
fn task_name_sort_orders_alphabetically() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    create_root_task(&mut manager, "Zulu");
    let middle = create_root_task(&mut manager, "Bravo");
    create_root_task(&mut manager, "Alpha");

    manager.sort_for_mode(&TaskSortMode::TaskName);

    let ordered: Vec<String> = manager
        .root
        .subtasks
        .iter()
        .map(|task| task.name.clone())
        .collect();
    assert_eq!(ordered, vec!["Alpha", "Bravo", "Zulu"]);
    assert_eq!(root_task(&manager, middle).name, "Bravo");
}

#[test]
fn task_sort_mode_parses_shared_codes() {
    assert_eq!(TaskSortMode::from_code("custom"), Some(TaskSortMode::Custom));
    assert_eq!(TaskSortMode::from_code("task_name"), Some(TaskSortMode::TaskName));
    assert_eq!(
        TaskSortMode::from_code("create_first"),
        Some(TaskSortMode::CreateFirst)
    );
    assert_eq!(
        TaskSortMode::from_code("update_first"),
        Some(TaskSortMode::UpdateFirst)
    );
    assert_eq!(
        TaskSortMode::from_code("complete_first"),
        Some(TaskSortMode::CompleteFirst)
    );
    assert_eq!(TaskSortMode::from_code("unknown"), None);
}
