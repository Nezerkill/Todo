use crate::format::print_task;
use crate::storage::JsonStorage;
use crate::storage::Storage;
use anyhow::{Context, Result};
use colored::Colorize;

pub fn execute(id: String) -> Result<()> {
    let storage = JsonStorage::new()?;

    // Находим задачу по полному ID или по короткому (первые 8 символов)
    let tasks = storage.load()?;

    let task = tasks.iter()
        .find(|t| t.id == id || t.id.starts_with(&id))
        .cloned()
        .context(format!("Задача с ID '{}' не найдена", id))?;

    let mut task = task;
    task.mark_done();

    storage.update(task.clone())
        .context("Не удалось обновить задачу")?;

    println!("{}", "✓ Задача выполнена".green());
    print_task(&task);

    Ok(())
}
