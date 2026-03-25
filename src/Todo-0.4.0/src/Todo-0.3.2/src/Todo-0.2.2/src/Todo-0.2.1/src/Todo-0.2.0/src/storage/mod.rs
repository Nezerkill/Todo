mod json;

pub use json::*;

use crate::models::Task;
use anyhow::Result;

pub trait Storage {
    fn load(&self) -> Result<Vec<Task>>;
    fn save(&self, tasks: &[Task]) -> Result<()>;
    fn add(&self, task: Task) -> Result<()>;
    fn update(&self, task: Task) -> Result<()>;
    fn remove(&self, id: &str) -> Result<bool>;
    fn get(&self, id: &str) -> Result<Option<Task>>;
}
