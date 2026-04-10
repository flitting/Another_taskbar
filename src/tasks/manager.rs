use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    filters::{ImportanceFilter, PinnedFilter, StateFilter, UrgencyFilter},
    model::{Task, TaskDraft, TaskState},
};

#[derive(Clone)]
struct TaskManagerSnapshot {
    root: Task,
    uni_id: u32,
    available_tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    pub root: Task,
    #[serde(default)]
    pub available_tags: Vec<String>,
    #[serde(skip)]
    pub active_filter_tags: Vec<String>,
    #[serde(skip)]
    pub active_importance_filter: ImportanceFilter,
    #[serde(skip)]
    pub active_urgency_filter: UrgencyFilter,
    #[serde(skip)]
    pub active_state_filter: StateFilter,
    #[serde(skip)]
    pub active_pinned_filter: PinnedFilter,
    #[serde(skip)]
    pub active_search_query: String,
    uni_id: u32,
    #[serde(skip)]
    last_snapshot: Option<TaskManagerSnapshot>,
}

impl TaskManager {
    pub fn new() -> Self {
        let mut manager = TaskManager {
            root: Task::empty_task(0),
            available_tags: Vec::new(),
            active_filter_tags: Vec::new(),
            active_importance_filter: ImportanceFilter::Any,
            active_urgency_filter: UrgencyFilter::Any,
            active_state_filter: StateFilter::Any,
            active_pinned_filter: PinnedFilter::Any,
            active_search_query: String::new(),
            uni_id: 0,
            last_snapshot: None,
        };
        manager.uni_id = manager.compute_max_id();
        manager
    }

    pub fn compute_max_id(&self) -> u32 {
        fn helper(task: &Task) -> u32 {
            let mut max = task.id;
            for subtask in &task.subtasks {
                let subtask_max = helper(subtask);
                if subtask_max > max {
                    max = subtask_max;
                }
            }
            max
        }

        helper(&self.root)
    }

    pub fn ensure_uni_id(&mut self) {
        let max = self.compute_max_id();
        if max > self.uni_id {
            self.uni_id = max;
        }
    }

    pub fn next_id(&mut self) -> u32 {
        self.ensure_uni_id();
        self.uni_id = self.uni_id.saturating_add(1);
        if self.uni_id == 0 {
            self.uni_id = 1;
        }
        self.uni_id
    }

    pub fn add_task(&mut self, id: u32, mut subtask: Task) -> Result<(), String> {
        if subtask.id == 0 {
            subtask.id = self.next_id();
        }

        let result = self.root.search_by_id(id);
        match result {
            Some(task) => {
                task.add_subtask(subtask);
                Ok(())
            }
            None => Err("can't find id in this field!".to_string()),
        }
    }

    fn resort_all_subtasks(task: &mut Task) {
        task.sort_subtasks();
        for subtask in &mut task.subtasks {
            Self::resort_all_subtasks(subtask);
        }
    }

    fn remember_state(&mut self) {
        self.last_snapshot = Some(TaskManagerSnapshot {
            root: self.root.clone(),
            uni_id: self.uni_id,
            available_tags: self.available_tags.clone(),
        });
    }

    pub fn can_undo(&self) -> bool {
        self.last_snapshot.is_some()
    }

    pub fn undo_last_change(&mut self) -> Result<(), String> {
        let snapshot = self
            .last_snapshot
            .take()
            .ok_or_else(|| "Nothing to undo".to_string())?;
        self.root = snapshot.root;
        self.uni_id = snapshot.uni_id;
        self.available_tags = snapshot.available_tags;
        Ok(())
    }

    fn normalize_available_tags(tags: Vec<String>) -> Vec<String> {
        let mut normalized = Vec::new();

        for tag in tags {
            let trimmed = tag.trim();
            if trimmed.is_empty() {
                continue;
            }

            if normalized.iter().any(|existing| existing == trimmed) {
                continue;
            }

            normalized.push(trimmed.to_string());
        }

        normalized
    }

    fn normalize_selected_tags(tags: Vec<String>) -> Vec<String> {
        let mut normalized = Vec::new();

        for tag in tags {
            let trimmed = tag.trim();
            if trimmed.is_empty() || normalized.iter().any(|existing| existing == trimmed) {
                continue;
            }

            normalized.push(trimmed.to_string());
        }

        normalized
    }

    pub fn set_available_tags(&mut self, tags: Vec<String>) {
        self.available_tags = Self::normalize_available_tags(tags);
    }

    pub fn has_active_filters(&self) -> bool {
        !self.active_filter_tags.is_empty()
            || !matches!(self.active_importance_filter, ImportanceFilter::Any)
            || !matches!(self.active_urgency_filter, UrgencyFilter::Any)
            || !matches!(self.active_state_filter, StateFilter::Any)
            || !matches!(self.active_pinned_filter, PinnedFilter::Any)
    }

    pub fn has_active_search(&self) -> bool {
        !self.active_search_query.trim().is_empty()
    }

    pub fn set_active_filter_tags(&mut self, tags: Vec<String>) {
        self.active_filter_tags = Self::normalize_available_tags(tags);
    }

    pub fn set_active_importance_filter(&mut self, filter: ImportanceFilter) {
        self.active_importance_filter = filter;
    }

    pub fn set_active_urgency_filter(&mut self, filter: UrgencyFilter) {
        self.active_urgency_filter = filter;
    }

    pub fn set_active_state_filter(&mut self, filter: StateFilter) {
        self.active_state_filter = filter;
    }

    pub fn set_active_pinned_filter(&mut self, filter: PinnedFilter) {
        self.active_pinned_filter = filter;
    }

    pub fn set_active_search_query(&mut self, query: String) {
        self.active_search_query = query.trim().to_string();
    }

    pub fn clear_active_search_query(&mut self) {
        self.active_search_query.clear();
    }

    pub fn clear_all_filters(&mut self) {
        self.active_filter_tags.clear();
        self.active_importance_filter = ImportanceFilter::Any;
        self.active_urgency_filter = UrgencyFilter::Any;
        self.active_state_filter = StateFilter::Any;
        self.active_pinned_filter = PinnedFilter::Any;
    }

    pub fn toggle_active_filter_tag(&mut self, tag: &str) {
        if self
            .active_filter_tags
            .iter()
            .any(|selected| selected == tag)
        {
            self.active_filter_tags.retain(|selected| selected != tag);
        } else if self.available_tags.iter().any(|available| available == tag) {
            self.active_filter_tags.push(tag.to_string());
            self.active_filter_tags =
                Self::normalize_available_tags(self.active_filter_tags.clone());
        }
    }

    pub fn most_common_tags(&self, limit: usize) -> Vec<String> {
        fn count_tags(task: &Task, counts: &mut HashMap<String, usize>) {
            for tag in &task.tags {
                *counts.entry(tag.clone()).or_insert(0) += 1;
            }

            for subtask in &task.subtasks {
                count_tags(subtask, counts);
            }
        }

        let mut counts = HashMap::new();
        count_tags(&self.root, &mut counts);

        let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
        ranked.sort_by(|(tag_a, count_a), (tag_b, count_b)| {
            count_b.cmp(count_a).then_with(|| tag_a.cmp(tag_b))
        });

        ranked.into_iter().map(|(tag, _)| tag).take(limit).collect()
    }

    pub fn clear_active_filter_tag(&mut self, tag: &str) {
        self.active_filter_tags.retain(|selected| selected != tag);
    }

    pub fn filtered_tasks(&self) -> Vec<Task> {
        self.root
            .subtasks
            .iter()
            .filter_map(|task| {
                task.filtered_clone(
                    &self.active_filter_tags,
                    &self.active_importance_filter,
                    &self.active_urgency_filter,
                    &self.active_state_filter,
                    &self.active_pinned_filter,
                    &self.active_search_query,
                )
            })
            .collect()
    }

    pub fn create_task_from_draft(
        &mut self,
        parent_id: u32,
        draft: TaskDraft,
    ) -> Result<u32, String> {
        if self.root.search_by_id_ref(parent_id).is_none() {
            return Err("can't find id in this field!".to_string());
        }
        self.remember_state();
        let task = draft.into_task(0);
        self.add_task(parent_id, task)?;
        Ok(self.uni_id)
    }

    pub fn update_task_from_draft(&mut self, id: u32, draft: TaskDraft) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        let task = self
            .root
            .search_by_id(id)
            .ok_or_else(|| "Task not found".to_string())?;

        task.apply_draft(draft);
        Self::resort_all_subtasks(&mut self.root);
        Ok(())
    }

    pub fn set_task_state(&mut self, id: u32, state: TaskState) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        let task = self
            .root
            .search_by_id(id)
            .ok_or_else(|| "Task not found".to_string())?;
        task.state = state;
        task.times.updated_at = Utc::now();
        Ok(())
    }

    pub fn toggle_task_pinned(&mut self, id: u32) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        if self.root.toggle_pinned(id) {
            Self::resort_all_subtasks(&mut self.root);
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn delete_task(&mut self, id: u32) -> Result<Task, String> {
        if id == 0 || self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        self.root
            .remove_subtask(id)
            .ok_or_else(|| "Task not found".to_string())
    }

    pub fn clear_tasks(&mut self) {
        self.remember_state();
        self.root = Task::empty_task(0);
        self.available_tags.clear();
        self.uni_id = 0;
    }

    pub fn save_task_detail(
        &mut self,
        target: Option<u32>,
        parent_id: u32,
        mut draft: TaskDraft,
        available_tags: Vec<String>,
    ) -> Result<Option<u32>, String> {
        let normalized_tags = Self::normalize_available_tags(available_tags);
        draft.tags = Self::normalize_selected_tags(draft.tags);
        draft
            .tags
            .retain(|tag| normalized_tags.iter().any(|known| known == tag));

        match target {
            Some(id) => {
                if self.root.search_by_id_ref(id).is_none() {
                    return Err("Task not found".to_string());
                }
                self.remember_state();
                self.set_available_tags(normalized_tags);
                let task = self
                    .root
                    .search_by_id(id)
                    .ok_or_else(|| "Task not found".to_string())?;
                task.apply_draft(draft);
                Self::resort_all_subtasks(&mut self.root);
                Ok(None)
            }
            None => {
                if self.root.search_by_id_ref(parent_id).is_none() {
                    return Err("can't find id in this field!".to_string());
                }
                self.remember_state();
                self.set_available_tags(normalized_tags);
                let task = draft.into_task(0);
                self.add_task(parent_id, task)?;
                Ok(Some(self.uni_id))
            }
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn matches_pinned_filter(pinned: bool, filter: &PinnedFilter) -> bool {
    match filter {
        PinnedFilter::Any => true,
        PinnedFilter::Pinned => pinned,
        PinnedFilter::Unpinned => !pinned,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::model::{TaskDraft, TaskState};

    #[test]
    fn undo_restores_last_created_task() {
        let mut manager = TaskManager::new();

        manager
            .create_task_from_draft(
                0,
                TaskDraft {
                    name: "Task".to_string(),
                    description: String::new(),
                    state: TaskState::Todo,
                    urgency: None,
                    importance: None,
                    tags: Vec::new(),
                    pinned: false,
                    due_date: None,
                    completed_at: None,
                },
            )
            .unwrap();

        assert_eq!(manager.root.subtasks.len(), 1);
        manager.undo_last_change().unwrap();
        assert!(manager.root.subtasks.is_empty());
    }

    #[test]
    fn undo_restores_last_deleted_task() {
        let mut manager = TaskManager::new();
        let task_id = manager
            .create_task_from_draft(
                0,
                TaskDraft {
                    name: "Task".to_string(),
                    description: String::new(),
                    state: TaskState::Todo,
                    urgency: None,
                    importance: None,
                    tags: Vec::new(),
                    pinned: false,
                    due_date: None,
                    completed_at: None,
                },
            )
            .unwrap();

        manager.delete_task(task_id).unwrap();
        assert!(manager.root.subtasks.is_empty());

        manager.undo_last_change().unwrap();
        assert_eq!(manager.root.subtasks.len(), 1);
        assert_eq!(manager.root.subtasks[0].id, task_id);
    }
}
