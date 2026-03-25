use crate::format::print_task;
use crate::models::{Priority, Task};
use crate::storage::JsonStorage;
use crate::storage::Storage;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;

pub fn execute(
    title: String,
    project: Option<String>,
    priority: Priority,
    due: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let storage = JsonStorage::new()?;

    let mut task = Task::new(title);

    if let Some(project) = project {
        task = task.with_project(project);
    }

    task = task.with_priority(priority);

    if !tags.is_empty() {
        task = task.with_tags(tags);
    }

    if let Some(due_str) = due {
        let due_date = parse_date(&due_str)
            .context(format!("Не удалось распарсить дату: {}", due_str))?;
        task = task.with_due_date(due_date);
    }

    storage.add(task.clone())
        .context("Не удалось добавить задачу")?;

    println!("{}", "✓ Задача добавлена".green());
    print_task(&task);

    Ok(())
}

fn parse_date(s: &str) -> Result<DateTime<Utc>> {
    // Пробуем разные форматы
    let formats = [
        "%Y-%m-%d",
        "%Y-%m-%d %H:%M",
        "%d.%m.%Y",
        "%d.%m.%Y %H:%M",
    ];

    for fmt in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Ok(dt.and_utc());
        }
    }

    // Только дата без времени
    for fmt in &["%Y-%m-%d", "%d.%m.%Y"] {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, fmt) {
            return Ok(date.and_hms_opt(23, 59, 0).unwrap().and_utc());
        }
    }

    anyhow::bail!("Неизвестный формат даты. Используйте: YYYY-MM-DD или DD.MM.YYYY")
}
