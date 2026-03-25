use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[default]
    Pending,
    Done,
    Deferred,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "pending"),
            Status::Done => write!(f, "done"),
            Status::Deferred => write!(f, "deferred"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub priority: Priority,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred_until: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            status: Status::Pending,
            priority: Priority::Medium,
            project: None,
            tags: Vec::new(),
            created_at: Utc::now(),
            due_date: None,
            completed_at: None,
            deferred_until: None,
        }
    }

    pub fn with_project(mut self, project: String) -> Self {
        self.project = Some(project);
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn mark_done(&mut self) {
        self.status = Status::Done;
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_pending(&mut self) {
        self.status = Status::Pending;
        self.completed_at = None;
    }

    pub fn mark_deferred(&mut self, until: DateTime<Utc>) {
        self.status = Status::Deferred;
        self.deferred_until = Some(until);
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            return self.status != Status::Done && Utc::now() > due;
        }
        false
    }
}
