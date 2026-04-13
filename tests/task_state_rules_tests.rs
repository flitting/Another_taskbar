mod common;

use another_taskbar::tasks::TaskState;

use common::draft_named;

#[test]
fn parent_auto_completes_when_all_direct_subtasks_complete() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let parent_id = manager
        .create_task_from_draft(0, draft_named("Parent"))
        .unwrap();
    let child_a = manager
        .create_task_from_draft(parent_id, draft_named("Child A"))
        .unwrap();
    let child_b = manager
        .create_task_from_draft(parent_id, draft_named("Child B"))
        .unwrap();

    manager
        .set_task_state_with_options(child_a, TaskState::Completed, false, true)
        .unwrap();
    manager
        .set_task_state_with_options(child_b, TaskState::Completed, false, true)
        .unwrap();

    let parent = manager.root.search_by_id_ref(parent_id).unwrap();
    assert_eq!(parent.state, TaskState::Completed);
}

#[test]
fn completing_parent_can_cascade_to_all_descendants() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let parent_id = manager
        .create_task_from_draft(0, draft_named("Parent"))
        .unwrap();
    let child_id = manager
        .create_task_from_draft(parent_id, draft_named("Child"))
        .unwrap();
    manager
        .create_task_from_draft(child_id, draft_named("Grandchild"))
        .unwrap();

    manager
        .set_task_state_with_options(parent_id, TaskState::Completed, true, true)
        .unwrap();

    let parent = manager.root.search_by_id_ref(parent_id).unwrap();
    let child = manager.root.search_by_id_ref(child_id).unwrap();
    let grandchild = child.subtasks.first().unwrap();
    assert_eq!(parent.state, TaskState::Completed);
    assert_eq!(child.state, TaskState::Completed);
    assert_eq!(grandchild.state, TaskState::Completed);
}

#[test]
fn blocking_parent_can_cascade_to_all_descendants() {
    let mut manager = another_taskbar::tasks::TaskManager::new();
    let parent_id = manager
        .create_task_from_draft(0, draft_named("Parent"))
        .unwrap();
    let child_id = manager
        .create_task_from_draft(parent_id, draft_named("Child"))
        .unwrap();

    manager
        .set_task_state_with_options(parent_id, TaskState::Blocked, true, true)
        .unwrap();

    let parent = manager.root.search_by_id_ref(parent_id).unwrap();
    let child = manager.root.search_by_id_ref(child_id).unwrap();
    assert_eq!(parent.state, TaskState::Blocked);
    assert_eq!(child.state, TaskState::Blocked);
}
