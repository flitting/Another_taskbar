use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use super::model::{Task, TaskState};

#[derive(Debug, Clone, Serialize)]
pub struct DueTaskNotification {
    pub task_id: u32,
    pub task_name: String,
    pub due_date: DateTime<Utc>,
    pub minutes_until_due: i64,
}

/// Find all tasks due within the next N minutes
pub fn find_tasks_due_soon(tasks: &[Task], minutes: i64) -> Vec<DueTaskNotification> {
    let now = Utc::now();
    let cutoff = now + Duration::minutes(minutes);

    let mut due_tasks = Vec::new();
    collect_due_tasks(tasks, now, cutoff, &mut due_tasks);

    // Sort by time until due (ascending)
    due_tasks.sort_by(|a, b| a.minutes_until_due.cmp(&b.minutes_until_due));

    due_tasks
}

fn collect_due_tasks(
    tasks: &[Task],
    now: DateTime<Utc>,
    cutoff: DateTime<Utc>,
    out: &mut Vec<DueTaskNotification>,
) {
    for task in tasks {
        // Only notify for tasks that are not completed or archived
        if task.state != TaskState::Completed && task.state != TaskState::Archived {
            if let Some(due_date) = task.times.due_date {
                // Check if due date is before cutoff and after now
                if due_date > now && due_date <= cutoff {
                    let minutes_until = (due_date - now).num_minutes();
                    out.push(DueTaskNotification {
                        task_id: task.id,
                        task_name: task.name.clone(),
                        due_date,
                        minutes_until_due: minutes_until,
                    });
                }
            }
        }

        // Recursively check subtasks
        collect_due_tasks(&task.subtasks, now, cutoff, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_tasks_due_soon() {
        let now = Utc::now();
        let in_5_mins = now + Duration::minutes(5);
        let in_1_hour = now + Duration::hours(1);

        let mut task_1 = Task::empty_task(1);
        task_1.name = "Task 1".to_string();
        task_1.state = TaskState::Todo;
        task_1.times.created_at = now;
        task_1.times.updated_at = now;
        task_1.times.due_date = Some(in_5_mins);

        let mut task_2 = Task::empty_task(2);
        task_2.name = "Task 2".to_string();
        task_2.state = TaskState::Todo;
        task_2.times.created_at = now;
        task_2.times.updated_at = now;
        task_2.times.due_date = Some(in_1_hour);

        let tasks = vec![task_1, task_2];

        let due_soon = find_tasks_due_soon(&tasks, 15);
        assert_eq!(due_soon.len(), 1);
        assert_eq!(due_soon[0].task_id, 1);
    }
}
