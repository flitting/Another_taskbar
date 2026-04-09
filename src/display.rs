use crate::tasks::*;

impl Task {
    pub fn display_single(&self) {
        // Display root task without connector
        let status_icon = self.get_status_icon();
        let pin_icon = if self.pinned { "📌 " } else { "   " };

        println!("{}{}", pin_icon, self.format_task_info(&status_icon));

        // Display all subtasks with tree structure
        for (index, subtask) in self.subtasks.iter().enumerate() {
            let is_last = index == self.subtasks.len() - 1;
            subtask.display_tree("", is_last);
        }
    }

    /// Display task as part of a tree structure
    fn display_tree(&self, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        let status_icon = self.get_status_icon();
        let pin_icon = if self.pinned { "📌 " } else { "   " };

        println!(
            "{}{}{}{}",
            prefix,
            connector,
            pin_icon,
            self.format_task_info(&status_icon)
        );

        // Display subtasks with extended prefix
        if !self.subtasks.is_empty() {
            let extension = if is_last { "    " } else { "│   " };
            for (index, subtask) in self.subtasks.iter().enumerate() {
                let is_last_subtask = index == self.subtasks.len() - 1;
                subtask.display_tree(&format!("{}{}", prefix, extension), is_last_subtask);
            }
        }
    }

    /// Get status icon based on task state
    fn get_status_icon(&self) -> &str {
        match self.state {
            TaskState::Todo => "○",
            TaskState::InProgress => "◐",
            TaskState::Blocked => "✗",
            TaskState::Completed => "✓",
            TaskState::Archived => "⇧",
        }
    }

    /// Format task information for display
    fn format_task_info(&self, status_icon: &str) -> String {
        format!("[{}] [{}] {}", status_icon, self.id, self.name)
    }

    /// Display detailed information about a task
    pub fn display_detail(&self) {
        let pin_icon = if self.pinned { " 📌" } else { "" };

        println!("{}{}", self.name, pin_icon);
        println!();

        if !self.description.is_empty() {
            println!("{}", self.description);
            println!();
        }

        let state_str = match self.state {
            TaskState::Todo => "TODO",
            TaskState::InProgress => "IN PROGRESS",
            TaskState::Blocked => "BLOCKED",
            TaskState::Completed => "COMPLETED",
            TaskState::Archived => "ARCHIVED",
        };
        println!("State: {}", state_str);

        let importance_str = self
            .importance
            .as_ref()
            .map(|i| match i {
                TaskImportance::Low => "LOW",
                TaskImportance::High => "HIGH",
            })
            .unwrap_or("NONE");

        let urgency_str = self
            .urgency
            .as_ref()
            .map(|u| match u {
                TaskUrgency::Low => "LOW",
                TaskUrgency::High => "HIGH",
            })
            .unwrap_or("NONE");

        println!("Importance: {} | Urgency: {}", importance_str, urgency_str);

        if !self.tags.is_empty() {
            println!("Tags: {}", self.tags.join(" | "));
        }

        println!();

        if let Some(due) = self.times.due_date {
            println!("Due time: {}", due.format("%Y-%m-%d %H:%M:%S"));
        } else {
            println!("Due time: Not set");
        }

        println!(
            "Created time: {}",
            self.times.created_at.format("%Y-%m-%d %H:%M:%S")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_task(id: u32, name: &str, state: TaskState, pinned: bool, layer: u32) -> Task {
        let times = TaskTimes {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            completed_at: None,
        };

        Task {
            id,
            name: name.to_string(),
            description: String::new(),
            state,
            urgency: None,
            importance: None,
            tags: Vec::new(),
            pinned,
            subtasks: Vec::new(),
            times,
            layer,
        }
    }

    #[test]
    fn test_display_single_todo_state() {
        let task = create_test_task(1, "Test Task", TaskState::Todo, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_in_progress_state() {
        let task = create_test_task(2, "In Progress Task", TaskState::InProgress, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_blocked_state() {
        let task = create_test_task(3, "Blocked Task", TaskState::Blocked, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_completed_state() {
        let task = create_test_task(4, "Completed Task", TaskState::Completed, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_archived_state() {
        let task = create_test_task(5, "Archived Task", TaskState::Archived, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_pinned_task() {
        let task = create_test_task(6, "Pinned Task", TaskState::Todo, true, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_unpinned_task() {
        let task = create_test_task(7, "Unpinned Task", TaskState::Todo, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_with_layer_indentation() {
        let task = create_test_task(8, "Layered Task", TaskState::Todo, false, 3);
        task.display_single();
    }

    #[test]
    fn test_display_single_with_subtasks() {
        let mut parent_task = create_test_task(9, "Parent Task", TaskState::Todo, false, 0);

        let subtask1 = create_test_task(10, "Subtask 1", TaskState::InProgress, false, 0);
        let subtask2 = create_test_task(11, "Subtask 2", TaskState::Completed, true, 0);

        parent_task.add_subtask(subtask1);
        parent_task.add_subtask(subtask2);

        parent_task.display_single();
    }

    #[test]
    fn test_display_single_with_nested_subtasks() {
        let mut root_task = create_test_task(12, "Root Task", TaskState::Todo, false, 0);

        let mut level1_subtask =
            create_test_task(13, "Level 1 Subtask", TaskState::InProgress, false, 0);
        let level2_subtask =
            create_test_task(14, "Level 2 Subtask", TaskState::Completed, false, 0);

        level1_subtask.add_subtask(level2_subtask);
        root_task.add_subtask(level1_subtask);

        root_task.display_single();
    }

    #[test]
    fn test_display_single_with_special_characters_in_name() {
        let task = create_test_task(15, "Task with 🎉 emoji", TaskState::Todo, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_with_long_task_name() {
        let long_name = "This is a very long task name that contains a lot of information about what needs to be done";
        let task = create_test_task(16, long_name, TaskState::Todo, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_single_multiple_states_and_pins() {
        let mut all_states_task =
            create_test_task(17, "Multi-state Parent", TaskState::Todo, true, 0);

        let tasks_to_add = vec![
            create_test_task(18, "Todo subtask", TaskState::Todo, false, 0),
            create_test_task(19, "InProgress pinned", TaskState::InProgress, true, 0),
            create_test_task(20, "Blocked subtask", TaskState::Blocked, false, 0),
            create_test_task(21, "Completed pinned", TaskState::Completed, true, 0),
            create_test_task(22, "Archived subtask", TaskState::Archived, false, 0),
        ];

        for task in tasks_to_add {
            all_states_task.add_subtask(task);
        }

        all_states_task.display_single();
    }

    #[test]
    fn test_display_with_zero_layer() {
        let task = create_test_task(23, "Zero Layer Task", TaskState::Todo, false, 0);
        task.display_single();
    }

    #[test]
    fn test_display_with_high_layer_number() {
        let task = create_test_task(24, "Deep Nested Task", TaskState::Todo, false, 10);
        task.display_single();
    }

    #[test]
    fn test_display_tree_format() {
        let mut father_task = create_test_task(1, "Father Task", TaskState::Todo, true, 0);

        let mut son_task1 = create_test_task(2, "Son Task 1", TaskState::InProgress, false, 0);
        let son_task2 = create_test_task(3, "Son Task 2", TaskState::Completed, true, 0);

        let sub_son_task = create_test_task(4, "Sub Son Task", TaskState::Todo, false, 0);
        son_task1.add_subtask(sub_son_task);

        father_task.add_subtask(son_task1);
        father_task.add_subtask(son_task2);

        father_task.display_single();
    }
}
