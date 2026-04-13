use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    filters::{ImportanceFilter, PinnedFilter, StateFilter, UrgencyFilter},
    manager::matches_pinned_filter,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TaskState {
    Todo,
    InProgress,
    Blocked,
    Completed,
    Archived,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TaskUrgency {
    Low,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TaskImportance {
    Low,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TaskSortMode {
    Custom,
    TaskName,
    CreateFirst,
    UpdateFirst,
    CompleteFirst,
}

impl TaskSortMode {
    pub fn all() -> [Self; 5] {
        [
            Self::Custom,
            Self::TaskName,
            Self::CreateFirst,
            Self::UpdateFirst,
            Self::CompleteFirst,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::TaskName => "task_name",
            Self::CreateFirst => "create_first",
            Self::UpdateFirst => "update_first",
            Self::CompleteFirst => "complete_first",
        }
    }

    pub fn from_code(value: &str) -> Option<Self> {
        match value {
            "custom" => Some(Self::Custom),
            "task_name" | "name" => Some(Self::TaskName),
            "create_first" | "created" => Some(Self::CreateFirst),
            "update_first" | "updated" => Some(Self::UpdateFirst),
            "complete_first" | "completed" => Some(Self::CompleteFirst),
            _ => None,
        }
    }
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            TaskState::Todo => "Todo",
            TaskState::InProgress => "In Progress",
            TaskState::Blocked => "Blocked",
            TaskState::Completed => "Completed",
            TaskState::Archived => "Archived",
        };

        write!(f, "{label}")
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RecurrenceFrequency {
    DoesNotRepeat,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Yearly,
    Custom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RecurrenceUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RecurrenceEnd {
    Never,
    OnDate(DateTime<Utc>),
    AfterOccurrences(u32),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CustomRecurrence {
    pub every: u32,
    pub unit: RecurrenceUnit,
    pub end: RecurrenceEnd,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecurrenceSetting {
    pub frequency: RecurrenceFrequency,
    pub due_hour: u8,
    pub due_minute: u8,
    pub custom: Option<CustomRecurrence>,
    pub occurrences_done: u32,
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
    pub layer: u32,
    #[serde(default)]
    pub custom_order: i64,
    #[serde(default)]
    pub recurrence: Option<RecurrenceSetting>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub recurrence: Option<RecurrenceSetting>,
}

impl Task {
    fn next_custom_order(children: &[Task]) -> i64 {
        children
            .iter()
            .map(|item| item.custom_order)
            .max()
            .unwrap_or(-1)
            .saturating_add(1)
    }

    fn normalize_task_tags(tags: Vec<String>) -> Vec<String> {
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

    pub fn empty_task(id: u32) -> Self {
        let now = Utc::now();
        let times = TaskTimes {
            created_at: now,
            updated_at: now,
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
            custom_order: 0,
            recurrence: None,
        }
    }

    pub fn add_subtask(&mut self, mut subtask: Task) {
        subtask.layer = self.layer + 1;
        subtask.custom_order = Self::next_custom_order(&self.subtasks);
        self.subtasks.push(subtask);
        self.sort_subtasks();
    }

    fn cmp_old_like(a: &Task, b: &Task) -> std::cmp::Ordering {
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
    }

    pub fn sort_subtasks(&mut self) {
        self.sort_subtasks_with_mode(&TaskSortMode::UpdateFirst);
    }

    pub fn sort_subtasks_with_mode(&mut self, mode: &TaskSortMode) {
        self.subtasks.sort_by(|a, b| match mode {
            TaskSortMode::Custom => a
                .custom_order
                .cmp(&b.custom_order)
                .then_with(|| Self::cmp_old_like(a, b)),
            TaskSortMode::TaskName => a
                .name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
                .then_with(|| Self::cmp_old_like(a, b)),
            TaskSortMode::CreateFirst => a
                .times
                .created_at
                .cmp(&b.times.created_at)
                .then_with(|| Self::cmp_old_like(a, b)),
            TaskSortMode::UpdateFirst => a
                .times
                .updated_at
                .cmp(&b.times.updated_at)
                .then_with(|| Self::cmp_old_like(a, b)),
            TaskSortMode::CompleteFirst => match (a.times.completed_at, b.times.completed_at) {
                (Some(a_at), Some(b_at)) => a_at.cmp(&b_at).then_with(|| Self::cmp_old_like(a, b)),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => Self::cmp_old_like(a, b),
            },
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

    pub fn apply_draft(&mut self, draft: TaskDraft) {
        self.name = draft.name;
        self.description = draft.description;
        self.state = draft.state;
        self.urgency = draft.urgency;
        self.importance = draft.importance;
        self.tags = Self::normalize_task_tags(draft.tags);
        self.pinned = draft.pinned;
        self.times.due_date = draft.due_date;
        self.times.completed_at = draft.completed_at;
        self.recurrence = draft.recurrence;
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

    pub(crate) fn filtered_clone(
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
            tags: Task::normalize_task_tags(self.tags),
            pinned: self.pinned,
            subtasks: Vec::new(),
            times: TaskTimes {
                created_at: now,
                updated_at: now,
                due_date: self.due_date,
                completed_at: self.completed_at,
            },
            layer: 0,
            custom_order: 0,
            recurrence: self.recurrence,
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
            recurrence: task.recurrence.clone(),
        }
    }
}
