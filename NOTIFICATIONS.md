# Task Notifications Feature

This document describes the task due notification system for Another Taskbar.

## Features

The notification system provides:
- Check for tasks due within a configurable time window (default: 15 minutes)
- Return lists of upcoming tasks sorted by time until due
- Track task ID, name, due date, and minutes remaining

## Backend Implementation

### Rust Module: `src/tasks/notifications.rs`

The module provides:
- `DueTaskNotification` - Struct representing a task due soon
- `find_tasks_due_soon(tasks, minutes)` - Find tasks due within N minutes

### Tauri Command: `check_upcoming_tasks`

Signature:
```rust
fn check_upcoming_tasks(
    state: State<'_, SharedState>,
    minutes_threshold: Option<i64>
) -> Result<Vec<DueTaskNotification>, String>
```

Uses default threshold of 15 minutes if not specified.

## Frontend Implementation

### JavaScript Integration

Add periodic checks to `ui/app.js`:

```javascript
// Check for upcoming tasks every 30 seconds
setInterval(async () => {
    try {
        const upcoming = await safeInvoke("check_upcoming_tasks", { 
            minutes_threshold: 15 
        });
        
        if (upcoming.length > 0) {
            // Show notification for each task
            upcoming.forEach(task => {
                showTaskNotification(task);
            });
        }
    } catch (error) {
        console.error("Failed to check upcoming tasks:", error);
    }
}, 30000);
```

### Desktop Notifications

To show desktop notifications, use Tauri's notification API (add to your backend):

```rust
// Add to Cargo.toml
tauri-plugin-notification = "2"

// In your Tauri command:
#[tauri::command]
fn notify_task_due(app_handle: AppHandle, task_name: String) -> Result<(), String> {
    tauri_plugin_notification::Notification::new(app_handle.config())
        .title("Task Due Soon")
        .body(&format!("{} is due in 15 minutes", task_name))
        .send()
        .map_err(|e| e.to_string())
}
```

## Usage Examples

### Check for tasks due in 15 minutes
```javascript
const upcoming = await invoke("check_upcoming_tasks", { minutes_threshold: 15 });
console.log(upcoming);
// Output: [
//   { task_id: 1, task_name: "Review PR", due_date: "...", minutes_until_due: 12 },
//   { task_id: 5, task_name: "Meeting", due_date: "...", minutes_until_due: 8 }
// ]
```

### Check for tasks due in 30 minutes
```javascript
const upcoming = await invoke("check_upcoming_tasks", { minutes_threshold: 30 });
```

### Use default (15 minutes)
```javascript
const upcoming = await invoke("check_upcoming_tasks");
```

## Future Enhancements

1. **Persistent Notification State**: Track which tasks have already been notified to avoid duplicates
2. **Sound Alerts**: Add optional sound notification for important tasks
3. **Notification Persistence**: Remember dismissed notifications
4. **Background Check Service**: Implement a background thread that periodically checks for upcoming tasks
5. **Custom Thresholds**: Allow users to configure notification time thresholds per task
6. **Snooze Functionality**: Allow users to snooze notifications

## Testing

Run tests with:
```bash
cargo test tasks::notifications
```

Example test:
```rust
#[test]
fn test_find_tasks_due_soon() {
    let now = Utc::now();
    let in_5_mins = now + Duration::minutes(5);
    
    let tasks = vec![
        Task {
            id: 1,
            name: "Urgent Task".to_string(),
            state: "Todo".to_string(),
            times: Some(TaskTimes {
                created_at: now,
                updated_at: now,
                due_date: Some(in_5_mins),
                completed_at: None,
            }),
            ..Default::default()
        },
    ];

    let due_soon = find_tasks_due_soon(&tasks, 15);
    assert_eq!(due_soon.len(), 1);
    assert_eq!(due_soon[0].task_id, 1);
    assert!(due_soon[0].minutes_until_due <= 5);
}
```

## Implementation Notes

- Only tasks with state `Todo`, `InProgress`, and `Blocked` are included
- Completed and Archived tasks are excluded from notifications
- Subtasks are checked recursively
- Times are in UTC, ensuring timezone-agnostic behavior
- Results are sorted by time until due (ascending)
