use crate::storage::JsonStorage;
use crate::storage::Storage;
use anyhow::{Context, Result};
use colored::Colorize;

pub fn execute(id: String) -> Result<()> {
    let storage = JsonStorage::new()?;

    // Находим задачу по полному ID или по короткому (первые 8 символов)
    let removed = storage.remove(&id)
        .context("Не удалось удалить задачу")?;

    if removed {
        println!("{}", "✓ Задача удалена".green());
    } else {
        println!("{}", 
            format!("Задача с ID '{}' не найдена", id).yellow()
        );
    }

    Ok(())
}
