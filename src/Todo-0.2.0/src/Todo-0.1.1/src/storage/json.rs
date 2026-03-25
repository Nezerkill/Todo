use crate::models::Task;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_local_dir()
            .context("Не удалось определить директорию данных")?
            .join("todo-rs");

        fs::create_dir_all(&data_dir)
            .context("Не удалось создать директорию данных")?;

        let path = data_dir.join("tasks.json");

        // Инициализируем файл, если он не существует
        if !path.exists() {
            fs::write(&path, "[]")
                .context("Не удалось создать файл задач")?;
        }

        Ok(Self { path })
    }

    fn read_tasks(&self) -> Result<Vec<Task>> {
        let content = fs::read_to_string(&self.path)
            .context("Не удалось прочитать файл задач")?;

        let tasks: Vec<Task> = serde_json::from_str(&content)
            .context("Не удалось распарсить JSON задач")?;

        Ok(tasks)
    }

    fn write_tasks(&self, tasks: &[Task]) -> Result<()> {
        let content = serde_json::to_string_pretty(tasks)
            .context("Не удалось сериализовать задачи")?;

        fs::write(&self.path, content)
            .context("Не удалось записать файл задач")?;

        Ok(())
    }
}

impl Storage for JsonStorage {
    fn load(&self) -> Result<Vec<Task>> {
        self.read_tasks()
    }

    fn save(&self, tasks: &[Task]) -> Result<()> {
        self.write_tasks(tasks)
    }

    fn add(&self, task: Task) -> Result<()> {
        let mut tasks = self.read_tasks()?;
        tasks.push(task);
        self.write_tasks(&tasks)
    }

    fn update(&self, task: Task) -> Result<()> {
        let mut tasks = self.read_tasks()?;

        let found = tasks.iter_mut().find(|t| t.id == task.id);

        if let Some(existing) = found {
            *existing = task;
            self.write_tasks(&tasks)?;
            Ok(())
        } else {
            anyhow::bail!("Задача с таким ID не найдена")
        }
    }

    fn remove(&self, id: &str) -> Result<bool> {
        let mut tasks = self.read_tasks()?;
        let len_before = tasks.len();

        tasks.retain(|t| t.id != id);

        if tasks.len() < len_before {
            self.write_tasks(&tasks)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get(&self, id: &str) -> Result<Option<Task>> {
        let tasks = self.read_tasks()?;
        Ok(tasks.into_iter().find(|t| t.id == id))
    }
}
