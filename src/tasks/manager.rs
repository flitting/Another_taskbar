use chrono::{Datelike, Duration, NaiveDate, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    filters::{ImportanceFilter, PinnedFilter, StateFilter, UrgencyFilter},
    model::{
        RecurrenceEnd, RecurrenceFrequency, RecurrenceSetting, RecurrenceUnit, Task, TaskDraft,
        TaskSortMode, TaskState,
    },
};

#[derive(Clone)]
struct TaskManagerSnapshot {
    root: Task,
    uni_id: u32,
    available_tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
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
    fn last_day_of_month(year: i32, month: u32) -> u32 {
        let next = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        };
        next.and_then(|date| date.pred_opt())
            .map(|date| date.day())
            .unwrap_or(28)
    }

    fn add_months_keep_day(date: chrono::DateTime<Utc>, months: i32) -> chrono::DateTime<Utc> {
        let naive = date.naive_utc();
        let mut year = naive.year();
        let mut month = naive.month() as i32 + months;
        while month > 12 {
            month -= 12;
            year += 1;
        }
        while month < 1 {
            month += 12;
            year -= 1;
        }
        let month_u = month as u32;
        let day = naive.day().min(Self::last_day_of_month(year, month_u));
        let fallback = date;
        let maybe = NaiveDate::from_ymd_opt(year, month_u, day)
            .and_then(|d| d.and_hms_opt(naive.hour(), naive.minute(), naive.second()));
        maybe
            .map(|ndt| chrono::DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
            .unwrap_or(fallback)
    }

    fn apply_due_clock(
        due: chrono::DateTime<Utc>,
        recurrence: &RecurrenceSetting,
    ) -> chrono::DateTime<Utc> {
        due.with_hour(u32::from(recurrence.due_hour))
            .and_then(|value| value.with_minute(u32::from(recurrence.due_minute)))
            .and_then(|value| value.with_second(0))
            .and_then(|value| value.with_nanosecond(0))
            .unwrap_or(due)
    }

    fn next_recurrence_due(
        due: chrono::DateTime<Utc>,
        recurrence: &RecurrenceSetting,
    ) -> chrono::DateTime<Utc> {
        let advanced = match recurrence.frequency {
            RecurrenceFrequency::DoesNotRepeat => due,
            RecurrenceFrequency::Daily => due + Duration::days(1),
            RecurrenceFrequency::Weekly => due + Duration::weeks(1),
            RecurrenceFrequency::Biweekly => due + Duration::weeks(2),
            RecurrenceFrequency::Monthly => Self::add_months_keep_day(due, 1),
            RecurrenceFrequency::Yearly => Self::add_months_keep_day(due, 12),
            RecurrenceFrequency::Custom => {
                if let Some(custom) = &recurrence.custom {
                    let every = custom.every.max(1) as i32;
                    match custom.unit {
                        RecurrenceUnit::Day => due + Duration::days(i64::from(every)),
                        RecurrenceUnit::Week => due + Duration::weeks(i64::from(every)),
                        RecurrenceUnit::Month => Self::add_months_keep_day(due, every),
                        RecurrenceUnit::Year => Self::add_months_keep_day(due, every * 12),
                    }
                } else {
                    due
                }
            }
        };
        Self::apply_due_clock(advanced, recurrence)
    }

    fn recurrence_end_reached(
        recurrence: &RecurrenceSetting,
        next_due: chrono::DateTime<Utc>,
    ) -> bool {
        match recurrence.frequency {
            RecurrenceFrequency::DoesNotRepeat => true,
            RecurrenceFrequency::Custom => {
                if let Some(custom) = &recurrence.custom {
                    match custom.end {
                        RecurrenceEnd::Never => false,
                        RecurrenceEnd::OnDate(limit) => next_due > limit,
                        RecurrenceEnd::AfterOccurrences(max_n) => {
                            recurrence.occurrences_done >= max_n
                        }
                    }
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    fn apply_recurrence_to_task(task: &mut Task, now: chrono::DateTime<Utc>) {
        if let (Some(recurrence), Some(mut due)) = (&mut task.recurrence, task.times.due_date) {
            if recurrence.frequency != RecurrenceFrequency::DoesNotRepeat {
                let original_due = due;
                while due <= now {
                    let next_due = Self::next_recurrence_due(due, recurrence);
                    if next_due <= due || Self::recurrence_end_reached(recurrence, next_due) {
                        break;
                    }
                    due = next_due;
                    recurrence.occurrences_done = recurrence.occurrences_done.saturating_add(1);
                }
                if due != original_due {
                    task.times.due_date = Some(due);
                    if task.state != TaskState::Blocked {
                        task.state = TaskState::Todo;
                    }
                    task.times.updated_at = now;
                }
            }
        }

        for child in &mut task.subtasks {
            Self::apply_recurrence_to_task(child, now);
        }
    }

    pub fn apply_recurring_updates(&mut self) {
        let now = Utc::now();
        Self::apply_recurrence_to_task(&mut self.root, now);
    }

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

    fn resort_all_subtasks(task: &mut Task, mode: &TaskSortMode) {
        task.sort_subtasks_with_mode(mode);
        for subtask in &mut task.subtasks {
            Self::resort_all_subtasks(subtask, mode);
        }
    }

    pub fn sort_for_mode(&mut self, mode: &TaskSortMode) {
        Self::resort_all_subtasks(&mut self.root, mode);
    }

    fn find_task_ref(task: &Task, id: u32) -> Option<&Task> {
        if task.id == id {
            return Some(task);
        }
        for child in &task.subtasks {
            if let Some(found) = Self::find_task_ref(child, id) {
                return Some(found);
            }
        }
        None
    }

    fn remove_task_from_tree(task: &mut Task, id: u32) -> Option<Task> {
        if let Some(index) = task.subtasks.iter().position(|item| item.id == id) {
            return Some(task.subtasks.remove(index));
        }
        for child in &mut task.subtasks {
            if let Some(found) = Self::remove_task_from_tree(child, id) {
                return Some(found);
            }
        }
        None
    }

    fn update_layer_recursive(task: &mut Task, layer: u32) {
        task.layer = layer;
        for child in &mut task.subtasks {
            Self::update_layer_recursive(child, layer + 1);
        }
    }

    fn resequence_custom_order(parent: &mut Task) {
        for (index, item) in parent.subtasks.iter_mut().enumerate() {
            item.custom_order = index as i64;
        }
    }

    fn move_to_parent_index(
        &mut self,
        task_id: u32,
        parent_id: u32,
        target_index: Option<usize>,
    ) -> Result<(), String> {
        if task_id == 0 || task_id == parent_id {
            return Err("Invalid move target".to_string());
        }
        let moving_ref =
            Self::find_task_ref(&self.root, task_id).ok_or_else(|| "Task not found".to_string())?;
        if Self::find_task_ref(moving_ref, parent_id).is_some() {
            return Err("Cannot move a task into its own descendant".to_string());
        }
        if self.root.search_by_id_ref(parent_id).is_none() {
            return Err("Parent task not found".to_string());
        }

        self.remember_state();
        let moving = Self::remove_task_from_tree(&mut self.root, task_id)
            .ok_or_else(|| "Task not found".to_string())?;
        let parent = self
            .root
            .search_by_id(parent_id)
            .ok_or_else(|| "Parent task not found".to_string())?;

        let idx = target_index
            .unwrap_or(parent.subtasks.len())
            .min(parent.subtasks.len());
        parent.subtasks.insert(idx, moving);
        Self::resequence_custom_order(parent);
        for child in &mut parent.subtasks {
            Self::update_layer_recursive(child, parent.layer + 1);
        }
        Ok(())
    }

    pub fn move_task_before(&mut self, task_id: u32, sibling_id: u32) -> Result<(), String> {
        if sibling_id == 0 {
            return self.move_to_parent_index(task_id, 0, Some(0));
        }
        let parent_id = self
            .find_parent_id(sibling_id)
            .ok_or_else(|| "Sibling task not found".to_string())?;
        let parent = self
            .root
            .search_by_id_ref(parent_id)
            .ok_or_else(|| "Parent task not found".to_string())?;
        let index = parent
            .subtasks
            .iter()
            .position(|item| item.id == sibling_id)
            .ok_or_else(|| "Sibling task not found".to_string())?;
        self.move_to_parent_index(task_id, parent_id, Some(index))
    }

    pub fn move_task_after(&mut self, task_id: u32, sibling_id: u32) -> Result<(), String> {
        if sibling_id == 0 {
            return self.move_to_parent_index(task_id, 0, None);
        }
        let parent_id = self
            .find_parent_id(sibling_id)
            .ok_or_else(|| "Sibling task not found".to_string())?;
        let parent = self
            .root
            .search_by_id_ref(parent_id)
            .ok_or_else(|| "Parent task not found".to_string())?;
        let index = parent
            .subtasks
            .iter()
            .position(|item| item.id == sibling_id)
            .ok_or_else(|| "Sibling task not found".to_string())?;
        self.move_to_parent_index(task_id, parent_id, Some(index + 1))
    }

    pub fn move_task_as_subtask(&mut self, task_id: u32, parent_id: u32) -> Result<(), String> {
        self.move_to_parent_index(task_id, parent_id, None)
    }

    fn find_parent_id_from(task: &Task, id: u32) -> Option<u32> {
        for child in &task.subtasks {
            if child.id == id {
                return Some(task.id);
            }
            if let Some(found) = Self::find_parent_id_from(child, id) {
                return Some(found);
            }
        }
        None
    }

    pub fn find_parent_id(&self, id: u32) -> Option<u32> {
        if id == 0 {
            return None;
        }
        Self::find_parent_id_from(&self.root, id)
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
        self.update_task_from_draft_with_options(id, draft, false, false)
    }

    pub fn update_task_from_draft_with_options(
        &mut self,
        id: u32,
        draft: TaskDraft,
        cascade_descendants: bool,
        auto_complete_parent_tasks: bool,
    ) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        let task = self
            .root
            .search_by_id(id)
            .ok_or_else(|| "Task not found".to_string())?;

        let state = draft.state.clone();
        task.apply_draft(draft);
        if cascade_descendants {
            let now = Utc::now();
            for child in &mut task.subtasks {
                Self::set_state_recursive(child, &state, now);
            }
        }
        if auto_complete_parent_tasks {
            self.apply_parent_completion_rollups();
        }
        Self::resort_all_subtasks(&mut self.root, &TaskSortMode::UpdateFirst);
        Ok(())
    }

    pub fn set_task_state(&mut self, id: u32, state: TaskState) -> Result<(), String> {
        self.set_task_state_with_options(id, state, false, false)
    }

    pub fn set_task_state_with_options(
        &mut self,
        id: u32,
        state: TaskState,
        cascade_descendants: bool,
        auto_complete_parent_tasks: bool,
    ) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        let now = Utc::now();
        let task = self
            .root
            .search_by_id(id)
            .ok_or_else(|| "Task not found".to_string())?;
        task.state = state.clone();
        task.times.updated_at = now;
        if cascade_descendants {
            for child in &mut task.subtasks {
                Self::set_state_recursive(child, &state, now);
            }
        }
        if auto_complete_parent_tasks {
            self.apply_parent_completion_rollups();
        }
        Ok(())
    }

    pub fn toggle_task_pinned(&mut self, id: u32) -> Result<(), String> {
        if self.root.search_by_id_ref(id).is_none() {
            return Err("Task not found".to_string());
        }
        self.remember_state();
        if self.root.toggle_pinned(id) {
            Self::resort_all_subtasks(&mut self.root, &TaskSortMode::UpdateFirst);
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
                Self::resort_all_subtasks(&mut self.root, &TaskSortMode::UpdateFirst);
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

    fn set_state_recursive(task: &mut Task, state: &TaskState, now: chrono::DateTime<Utc>) {
        task.state = state.clone();
        task.times.updated_at = now;
        for child in &mut task.subtasks {
            Self::set_state_recursive(child, state, now);
        }
    }

    fn apply_parent_completion_rollups_recursive(
        task: &mut Task,
        now: chrono::DateTime<Utc>,
    ) -> bool {
        for child in &mut task.subtasks {
            Self::apply_parent_completion_rollups_recursive(child, now);
        }

        if !task.subtasks.is_empty()
            && task
                .subtasks
                .iter()
                .all(|child| matches!(child.state, TaskState::Completed))
            && task.state != TaskState::Completed
        {
            task.state = TaskState::Completed;
            task.times.updated_at = now;
        }

        matches!(task.state, TaskState::Completed)
    }

    pub fn apply_parent_completion_rollups(&mut self) {
        let now = Utc::now();
        Self::apply_parent_completion_rollups_recursive(&mut self.root, now);
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
                    recurrence: None,
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
                    recurrence: None,
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
