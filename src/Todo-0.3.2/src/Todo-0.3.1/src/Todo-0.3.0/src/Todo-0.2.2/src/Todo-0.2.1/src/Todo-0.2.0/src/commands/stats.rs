use crate::models::Status;
use crate::storage::JsonStorage;
use crate::storage::Storage;
use anyhow::Result;
use colored::Colorize;

pub fn execute() -> Result<()> {
    let storage = JsonStorage::new()?;
    let tasks = storage.load()?;

    let total = tasks.len();
    let pending = tasks.iter().filter(|t| t.status == Status::Pending).count();
    let done = tasks.iter().filter(|t| t.status == Status::Done).count();
    let deferred = tasks.iter().filter(|t| t.status == Status::Deferred).count();
    let overdue = tasks.iter().filter(|t| t.is_overdue()).count();

    // По проектам
    let mut by_project: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for task in &tasks {
        if let Some(proj) = &task.project {
            *by_project.entry(proj.clone()).or_insert(0) += 1;
        }
    }

    // По приоритетам
    let high = tasks.iter().filter(|t| matches!(t.priority, crate::models::Priority::High)).count();
    let medium = tasks.iter().filter(|t| matches!(t.priority, crate::models::Priority::Medium)).count();
    let low = tasks.iter().filter(|t| matches!(t.priority, crate::models::Priority::Low)).count();

    println!("\n{}", "📊 Статистика задач".bold().blue());
    println!("{}", "═".repeat(40));

    println!("\n{}", "По статусу:".bold());
    println!("  {:<20} {}", "Всего:", total.to_string().bold());
    println!("  {:<20} {}", "⬜ В ожидании:", pending.to_string().yellow());
    println!("  {:<20} {}", "✅ Выполнено:", done.to_string().green());
    println!("  {:<20} {}", "⏸️ Отложено:", deferred.to_string().cyan());
    println!("  {:<20} {}", "🔴 Просрочено:", overdue.to_string().red());

    println!("\n{}", "По приоритету:".bold());
    println!("  {:<20} {}", "🔴 Высокий:", high.to_string().red());
    println!("  {:<20} {}", "🟡 Средний:", medium.to_string().yellow());
    println!("  {:<20} {}", "🟢 Низкий:", low.to_string().green());

    if !by_project.is_empty() {
        println!("\n{}", "По проектам:".bold());
        let mut projects: Vec<_> = by_project.iter().collect();
        projects.sort_by(|a, b| b.1.cmp(a.1));
        for (proj, count) in projects {
            println!("  {:<20} {}", proj, count);
        }
    }

    // Процент выполнения
    if total > 0 {
        let percent = (done as f64 / total as f64) * 100.0;
        println!("\n{}", "Прогресс:".bold());
        println!("  {:<20} {:.1}%", "Выполнено:", percent);
    }

    println!();

    Ok(())
}
