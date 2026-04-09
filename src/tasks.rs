use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskState {
    Todo,
    InProgress,
    Blocked,
    Completed,
    Archived,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskUrgency {
    Low,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskImportance {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportanceFilter {
    Any,
    High,
    Low,
    Neither,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UrgencyFilter {
    Any,
    High,
    Low,
    Neither,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PinnedFilter {
    Any,
    Pinned,
    Unpinned,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateFilter {
    Any,
    Todo,
    InProgress,
    Blocked,
    Completed,
    Archived,
    None,
}

impl Default for ImportanceFilter {
    fn default() -> Self {
        Self::Any
    }
}

impl Default for UrgencyFilter {
    fn default() -> Self {
        Self::Any
    }
}

impl Default for PinnedFilter {
    fn default() -> Self {
        Self::Any
    }
}

impl Default for StateFilter {
    fn default() -> Self {
        Self::Any
    }
}

impl std::fmt::Display for TaskUrgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            TaskUrgency::Low => "Low Urgency",
            TaskUrgency::High => "High Urgency",
        };

        write!(f, "{label}")
    }
}

impl std::fmt::Display for TaskImportance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            TaskImportance::Low => "Low Importance",
            TaskImportance::High => "High Importance",
        };

        write!(f, "{label}")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskTimes {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub state: TaskState,
    pub urgency: Option<TaskUrgency>,
    pub importance: Option<TaskImportance>,
    pub tags: Vec<String>,
    pub pinned: bool,
    pub subtasks: Vec<Task>,
    pub times: TaskTimes,
    // the sub task layer for the task, default is 0.
    pub layer: u32,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TaskUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<TaskState>,
    pub urgency: Option<TaskUrgency>,
    pub importance: Option<TaskImportance>,
    pub tags: Option<Vec<String>>,
    pub pinned: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct TaskDraft {
    pub name: String,
    pub description: String,
    pub state: TaskState,
    pub urgency: Option<TaskUrgency>,
    pub importance: Option<TaskImportance>,
    pub tags: Vec<String>,
    pub pinned: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(
        id: u32,
        name: String,
        description: String,
        state: TaskState,
        urgency: Option<TaskUrgency>,
        importance: Option<TaskImportance>,
        tags: Vec<String>,
        pinned: bool,
        times: TaskTimes,
        layer: u32,
    ) -> Self {
        Task {
            id,
            name,
            description,
            state,
            urgency,
            importance,
            tags,
            pinned,
            subtasks: Vec::new(),
            times,
            layer,
        }
    }

    pub fn empty_task(id: u32) -> Self {
        let times = TaskTimes {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            completed_at: None,
        };

        Task {
            id,
            name: String::new(),
            description: String::new(),
            state: TaskState::Todo,
            urgency: None,
            importance: None,
            tags: Vec::new(),
            subtasks: Vec::new(),
            pinned: false,
            times,
            layer: 0,
        }
    }
    pub fn add_subtask(&mut self, mut subtask: Task) {
        subtask.layer = self.layer + 1;
        self.subtasks.push(subtask);
        self.sort_subtasks();
    }
    // sort subtasks with pinned first by the order of due time.Put task without due time below sorted by created time.
    pub fn sort_subtasks(&mut self) {
        self.subtasks.sort_by(|a, b| {
            if a.pinned && !b.pinned {
                std::cmp::Ordering::Less
            } else if !a.pinned && b.pinned {
                std::cmp::Ordering::Greater
            } else {
                match (a.times.due_date, b.times.due_date) {
                    (Some(a_due), Some(b_due)) => a_due.cmp(&b_due),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.times.updated_at.cmp(&b.times.updated_at),
                }
            }
        });
    }
    pub fn remove_subtask(&mut self, id: u32) -> Option<Task> {
        if let Some(index) = self.subtasks.iter().position(|s| s.id == id) {
            let removed = self.subtasks.remove(index);
            self.sort_subtasks();
            return Some(removed);
        }
        for subtask in &mut self.subtasks {
            if let Some(removed) = subtask.remove_subtask(id) {
                self.sort_subtasks();
                return Some(removed);
            }
        }
        None
    }

    pub fn search_by_id(&mut self, id: u32) -> Option<&mut Task> {
        if self.id == id {
            return Some(self);
        }
        for subtask in &mut self.subtasks {
            if let Some(found) = subtask.search_by_id(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn search_by_id_ref(&self, id: u32) -> Option<&Task> {
        if self.id == id {
            return Some(self);
        }
        for subtask in &self.subtasks {
            if let Some(found) = subtask.search_by_id_ref(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn toggle_pinned(&mut self, id: u32) -> bool {
        if self.id == id {
            self.pinned = !self.pinned;
            self.times.updated_at = Utc::now();
            return true;
        }

        for subtask in &mut self.subtasks {
            if subtask.toggle_pinned(id) {
                self.sort_subtasks();
                return true;
            }
        }

        false
    }

    pub fn change_field(&mut self, update: TaskUpdate) {
        if let Some(v) = update.name {
            self.name = v;
        }
        if let Some(v) = update.description {
            self.description = v;
        }
        if let Some(v) = update.state {
            self.state = v;
        }
        if let Some(v) = update.urgency {
            self.urgency = Some(v);
        }
        if let Some(v) = update.importance {
            self.importance = Some(v);
        }
        if let Some(v) = update.tags {
            self.tags = v;
        }
        if let Some(v) = update.pinned {
            self.pinned = v;
        }
        self.times.updated_at = Utc::now();
        if let Some(v) = update.due_date {
            self.times.due_date = Some(v);
        }
    }

    pub fn apply_draft(&mut self, draft: TaskDraft) {
        self.name = draft.name;
        self.description = draft.description;
        self.state = draft.state;
        self.urgency = draft.urgency;
        self.importance = draft.importance;
        self.tags = draft.tags;
        self.pinned = draft.pinned;
        self.times.due_date = draft.due_date;
        self.times.completed_at = draft.completed_at;
        self.times.updated_at = Utc::now();
    }

    fn matches_any_filter_tag(&self, selected_tags: &[String]) -> bool {
        self.tags
            .iter()
            .any(|tag| selected_tags.iter().any(|selected| selected == tag))
    }

    fn matches_importance_filter(&self, filter: &ImportanceFilter) -> bool {
        match filter {
            ImportanceFilter::Any => true,
            ImportanceFilter::High => matches!(self.importance, Some(TaskImportance::High)),
            ImportanceFilter::Low => matches!(self.importance, Some(TaskImportance::Low)),
            ImportanceFilter::Neither => self.importance.is_none(),
        }
    }

    fn matches_urgency_filter(&self, filter: &UrgencyFilter) -> bool {
        match filter {
            UrgencyFilter::Any => true,
            UrgencyFilter::High => matches!(self.urgency, Some(TaskUrgency::High)),
            UrgencyFilter::Low => matches!(self.urgency, Some(TaskUrgency::Low)),
            UrgencyFilter::Neither => self.urgency.is_none(),
        }
    }

    fn matches_state_filter(&self, filter: &StateFilter) -> bool {
        match filter {
            StateFilter::Any => true,
            StateFilter::Todo => matches!(self.state, TaskState::Todo),
            StateFilter::InProgress => matches!(self.state, TaskState::InProgress),
            StateFilter::Blocked => matches!(self.state, TaskState::Blocked),
            StateFilter::Completed => matches!(self.state, TaskState::Completed),
            StateFilter::Archived => matches!(self.state, TaskState::Archived),
            StateFilter::None => false,
        }
    }

    fn matches_filter_state(
        &self,
        selected_tags: &[String],
        importance_filter: &ImportanceFilter,
        urgency_filter: &UrgencyFilter,
        state_filter: &StateFilter,
        pinned_filter: &PinnedFilter,
        search_query: &str,
    ) -> bool {
        let tags_match = selected_tags.is_empty() || self.matches_any_filter_tag(selected_tags);
        let normalized_search = search_query.trim().to_lowercase();
        let search_match = normalized_search.is_empty()
            || self.name.to_lowercase().contains(&normalized_search)
            || self.description.to_lowercase().contains(&normalized_search);
        tags_match
            && self.matches_importance_filter(importance_filter)
            && self.matches_urgency_filter(urgency_filter)
            && self.matches_state_filter(state_filter)
            && matches_pinned_filter(self.pinned, pinned_filter)
            && search_match
    }

    fn filtered_clone(
        &self,
        selected_tags: &[String],
        importance_filter: &ImportanceFilter,
        urgency_filter: &UrgencyFilter,
        state_filter: &StateFilter,
        pinned_filter: &PinnedFilter,
        search_query: &str,
    ) -> Option<Task> {
        let no_filters = selected_tags.is_empty()
            && matches!(importance_filter, ImportanceFilter::Any)
            && matches!(urgency_filter, UrgencyFilter::Any)
            && matches!(state_filter, StateFilter::Any)
            && matches!(pinned_filter, PinnedFilter::Any)
            && search_query.trim().is_empty();
        if no_filters {
            return Some(self.clone());
        }

        let filtered_subtasks: Vec<Task> = self
            .subtasks
            .iter()
            .filter_map(|subtask| {
                subtask.filtered_clone(
                    selected_tags,
                    importance_filter,
                    urgency_filter,
                    state_filter,
                    pinned_filter,
                    search_query,
                )
            })
            .collect();

        if self.matches_filter_state(
            selected_tags,
            importance_filter,
            urgency_filter,
            state_filter,
            pinned_filter,
            search_query,
        ) || !filtered_subtasks.is_empty()
        {
            let mut filtered = self.clone();
            filtered.subtasks = filtered_subtasks;
            Some(filtered)
        } else {
            None
        }
    }
}

impl TaskDraft {
    pub fn into_task(self, id: u32) -> Task {
        let now = Utc::now();

        Task {
            id,
            name: self.name,
            description: self.description,
            state: self.state,
            urgency: self.urgency,
            importance: self.importance,
            tags: self.tags,
            pinned: self.pinned,
            subtasks: Vec::new(),
            times: TaskTimes {
                created_at: now,
                updated_at: now,
                due_date: self.due_date,
                completed_at: self.completed_at,
            },
            layer: 0,
        }
    }
}

impl From<&Task> for TaskDraft {
    fn from(task: &Task) -> Self {
        Self {
            name: task.name.clone(),
            description: task.description.clone(),
            state: task.state.clone(),
            urgency: task.urgency.clone(),
            importance: task.importance.clone(),
            tags: task.tags.clone(),
            pinned: task.pinned,
            due_date: task.times.due_date,
            completed_at: task.times.completed_at,
        }
    }
}

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
        let mut m = TaskManager {
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
        // initialize uni_id to the current maximum id in the tree so next_id() won't collide
        m.uni_id = m.compute_max_id();
        m
    }

    /// Compute the maximum id currently present in the task tree.
    pub fn compute_max_id(&self) -> u32 {
        fn helper(task: &Task) -> u32 {
            let mut max = task.id;
            for s in &task.subtasks {
                let m = helper(s);
                if m > max {
                    max = m;
                }
            }
            max
        }
        helper(&self.root)
    }

    /// Ensure internal counter is at least the current maximum id.
    pub fn ensure_uni_id(&mut self) {
        let max = self.compute_max_id();
        if max > self.uni_id {
            self.uni_id = max;
        }
    }

    /// Generate the next unique id.
    pub fn next_id(&mut self) -> u32 {
        // ensure counter not behind existing ids
        self.ensure_uni_id();
        self.uni_id = self.uni_id.saturating_add(1);
        if self.uni_id == 0 {
            // guarding against wrap (extremely unlikely)
            self.uni_id = 1;
        }
        self.uni_id
    }

    // assign the subtask under the id, 0 for the root
    pub fn add_task(&mut self, id: u32, mut subtask: Task) -> Result<(), String> {
        // Ensure the subtask has a unique id before borrowing into the tree.
        // Calling `next_id()` mutably borrows `self`, so do that first.
        if subtask.id == 0 {
            subtask.id = self.next_id();
        }

        let result = self.root.search_by_id(id);
        match result {
            Some(task) => {
                task.add_subtask(subtask);
                return Ok(());
            }
            None => return Err("can't find id in this field!".to_string()),
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

    fn normalize_tags(tags: Vec<String>) -> Vec<String> {
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
            if normalized.len() == 3 {
                break;
            }
        }

        normalized
    }

    pub fn set_available_tags(&mut self, tags: Vec<String>) {
        self.available_tags = Self::normalize_tags(tags);
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
        self.active_filter_tags = Self::normalize_tags(tags);
    }

    pub fn clear_active_filter_tags(&mut self) {
        self.active_filter_tags.clear();
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
            self.active_filter_tags = Self::normalize_tags(self.active_filter_tags.clone());
        }
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
        let normalized_tags = Self::normalize_tags(available_tags);
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
                let id = self.uni_id;
                Ok(Some(id))
            }
        }
    }
}

fn matches_pinned_filter(pinned: bool, filter: &PinnedFilter) -> bool {
    match filter {
        PinnedFilter::Any => true,
        PinnedFilter::Pinned => pinned,
        PinnedFilter::Unpinned => !pinned,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
