use crate::format::print_tasks;
use crate::models::{Priority, Status};
use crate::storage::JsonStorage;
use crate::storage::Storage;
use anyhow::Result;

pub fn execute(
    project: Option<String>,
    priority: Option<Priority>,
    status: Option<Status>,
    overdue: bool,
    pending: bool,
    done: bool,
) -> Result<()> {
    let storage = JsonStorage::new()?;
    let mut tasks = storage.load()?;

    // Фильтрация по проекту
    if let Some(proj) = project {
        tasks.retain(|t| t.project.as_ref().map_or(false, |p| p == &proj));
    }

    // Фильтрация по приоритету
    if let Some(pri) = priority {
        tasks.retain(|t| t.priority == pri);
    }

    // Фильтрация по статусу
    if let Some(st) = status {
        tasks.retain(|t| t.status == st);
    }

    // Фильтры-ярлыки
    if pending {
        tasks.retain(|t| t.status == Status::Pending);
    }

    if done {
        tasks.retain(|t| t.status == Status::Done);
    }

    // Просроченные
    if overdue {
        tasks.retain(|t| t.is_overdue());
    }

    // Сортировка: сначала невыполненные, потом по приоритету
    tasks.sort_by(|a, b| {
        // Сначала невыполненные
        let status_order = |s: &Status| match s {
            Status::Pending => 0,
            Status::Deferred => 1,
            Status::Done => 2,
        };
        status_order(&a.status).cmp(&status_order(&b.status))
            .then_with(|| {
                // Потом по приоритету (high > medium > low)
                let priority_order = |p: &Priority| match p {
                    Priority::High => 0,
                    Priority::Medium => 1,
                    Priority::Low => 2,
                };
                priority_order(&a.priority).cmp(&priority_order(&b.priority))
            })
            .then_with(|| {
                // Потом по дедлайну
                a.due_date.cmp(&b.due_date)
            })
    });

    print_tasks(&tasks);

    Ok(())
}
