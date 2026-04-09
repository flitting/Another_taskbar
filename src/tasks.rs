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
}

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    pub root: Task,
    uni_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        let mut m = TaskManager {
            root: Task::empty_task(0),
            uni_id: 0,
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
}
