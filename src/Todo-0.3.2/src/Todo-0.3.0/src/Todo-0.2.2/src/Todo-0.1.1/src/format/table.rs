use crate::models::{Priority, Status, Task};
use colored::Colorize;
use comfy_table::{Table, ContentArrangement};
use chrono::Utc;

pub fn format_priority(priority: &Priority) -> String {
    match priority {
        Priority::High => "🔴 high".red().to_string(),
        Priority::Medium => "🟡 medium".yellow().to_string(),
        Priority::Low => "🟢 low".green().to_string(),
    }
}

pub fn format_status(status: &Status) -> String {
    match status {
        Status::Pending => "⬜ pending".white().to_string(),
        Status::Done => "✅ done".green().to_string(),
        Status::Deferred => "⏸️ deferred".cyan().to_string(),
    }
}

pub fn format_date(date: &Option<chrono::DateTime<Utc>>) -> String {
    match date {
        Some(d) => d.format("%Y-%m-%d").to_string(),
        None => "—".to_string(),
    }
}

pub fn format_task_short(task: &Task) -> String {
    let id = &task.id[..8];
    let title = &task.title;
    let project = task.project.as_deref().unwrap_or("—");
    let priority = format_priority(&task.priority);
    let status = format_status(&task.status);

    format!("[{}] {} | {} | {} | {}",
        id.bold(),
        title,
        project,
        priority,
        status
    )
}

pub fn create_tasks_table(tasks: &[Task]) -> Table {
    let mut table = Table::new();
    
    table
        .set_header(vec!["ID", "Задача", "Проект", "Приоритет", "Статус", "Дедлайн"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(120);

    for task in tasks {
        let id = &task.id[..8];
        let priority_str = match task.priority {
            Priority::High => "🔴 high",
            Priority::Medium => "🟡 medium",
            Priority::Low => "🟢 low",
        };

        let status_str = match task.status {
            Status::Pending => "⬜ pending",
            Status::Done => "✅ done",
            Status::Deferred => "⏸️ deferred",
        };

        let date_str = format_date(&task.due_date);

        table.add_row(vec![
            id,
            &task.title,
            task.project.as_deref().unwrap_or("—"),
            priority_str,
            status_str,
            &date_str,
        ]);
    }

    table
}

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "Задач не найдено".italic());
        return;
    }

    let table = create_tasks_table(tasks);
    println!("{table}");
}

pub fn print_task(task: &Task) {
    println!("\n{}", "═".repeat(60));
    println!("{} {}", "Задача:".bold().blue(), task.title);
    println!("{} {}", "ID:".bold(), &task.id[..8]);
    println!("{} {}", "Статус:".bold(), format_status(&task.status));
    println!("{} {}", "Приоритет:".bold(), format_priority(&task.priority));
    
    if let Some(project) = &task.project {
        println!("{} {}", "Проект:".bold(), project);
    }
    
    if !task.tags.is_empty() {
        println!("{} {}", "Теги:".bold(), task.tags.join(", "));
    }
    
    println!("{} {}", "Создано:".bold(), task.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(due) = task.due_date {
        let overdue = if task.is_overdue() { " (просрочено!)".red().to_string() } else { String::new() };
        println!("{} {}{}", "Дедлайн:".bold(), due.format("%Y-%m-%d"), overdue);
    }
    
    if let Some(completed) = task.completed_at {
        println!("{} {}", "Выполнено:".bold(), completed.format("%Y-%m-%d %H:%M"));
    }
    
    println!("{}\n", "═".repeat(60));
}
