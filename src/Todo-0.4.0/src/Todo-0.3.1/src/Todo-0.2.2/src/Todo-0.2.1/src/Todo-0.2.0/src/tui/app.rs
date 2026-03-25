use crate::models::{Priority, Status, Task};
use crate::storage::{JsonStorage, Storage};
use anyhow::Result;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Add,
    Search,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub project: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub search: Option<String>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            project: None,
            priority: None,
            status: Some(Status::Pending),
            search: None,
        }
    }
}

pub struct App {
    pub tasks: Vec<Task>,
    pub selected: Option<usize>,
    pub scroll: usize,
    pub mode: InputMode,
    pub filter: Filter,
    pub input: String,
    pub cursor: usize,
    pub message: Option<(String, Instant)>,
    storage: JsonStorage,
}

impl App {
    pub fn new() -> Result<Self> {
        let storage = JsonStorage::new()?;
        let tasks = storage.load()?;

        Ok(Self {
            tasks,
            selected: None,
            scroll: 0,
            mode: InputMode::Normal,
            filter: Filter::default(),
            input: String::new(),
            cursor: 0,
            message: None,
            storage,
        })
    }

    pub fn filtered_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| {
                if let Some(ref status) = self.filter.status {
                    if &t.status != status {
                        return false;
                    }
                }
                if let Some(ref project) = self.filter.project {
                    if t.project.as_ref() != Some(project) {
                        return false;
                    }
                }
                if let Some(ref priority) = self.filter.priority {
                    if &t.priority != priority {
                        return false;
                    }
                }
                if let Some(ref search) = self.filter.search {
                    if !t.title.to_lowercase().contains(&search.to_lowercase()) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn select_next(&mut self) {
        let filtered = self.filtered_tasks();
        if filtered.is_empty() {
            return;
        }

        let current_idx = self.selected.unwrap_or(0);
        let next_idx = (current_idx + 1).min(filtered.len() - 1);
        self.selected = Some(next_idx);
    }

    pub fn select_previous(&mut self) {
        if self.selected.unwrap_or(0) > 0 {
            self.selected = Some(self.selected.unwrap() - 1);
        }
    }

    pub fn select_first(&mut self) {
        if !self.filtered_tasks().is_empty() {
            self.selected = Some(0);
        }
    }

    pub fn select_last(&mut self) {
        let len = self.filtered_tasks().len();
        if len > 0 {
            self.selected = Some(len - 1);
        }
    }

    pub fn selected_task(&self) -> Option<&Task> {
        let filtered = self.filtered_tasks();
        self.selected
            .and_then(|idx| filtered.get(idx).copied())
    }

    pub fn selected_task_id(&self) -> Option<String> {
        self.selected_task().map(|t| t.id.clone())
    }

    pub fn mark_done(&mut self) -> Result<()> {
        if let Some(id) = self.selected_task_id() {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.mark_done();
                self.storage.update(task.clone())?;
                self.tasks = self.storage.load()?;
                self.set_message("✓ Задача выполнена");
            }
        }
        Ok(())
    }

    pub fn delete_task(&mut self) -> Result<()> {
        if let Some(id) = self.selected_task_id() {
            self.storage.remove(&id)?;
            self.tasks = self.storage.load()?;
            self.set_message("✓ Задача удалена");

            if self.selected.is_some() {
                let len = self.filtered_tasks().len();
                if len == 0 {
                    self.selected = None;
                } else {
                    self.selected = Some(self.selected.unwrap().min(len - 1));
                }
            }
        }
        Ok(())
    }

    pub fn add_task(&mut self, title: String, priority: Priority, project: Option<String>) -> Result<()> {
        let task = Task::new(title)
            .with_priority(priority)
            .with_project(project.unwrap_or_default());

        self.storage.add(task.clone())?;
        self.tasks = self.storage.load()?;
        self.set_message("✓ Задача добавлена");
        Ok(())
    }

    pub fn toggle_all_filter(&mut self) {
        self.filter.status = if self.filter.status.is_some() {
            None
        } else {
            Some(Status::Pending)
        };
    }

    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), Instant::now()));
    }

    pub fn tick(&mut self) {
        if let Some((_, time)) = &self.message {
            if time.elapsed().as_secs() > 2 {
                self.message = None;
            }
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        self.tasks = self.storage.load()?;
        Ok(())
    }
}
