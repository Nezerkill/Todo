mod commands;
mod format;
mod models;
mod storage;
mod tui;

use clap::{Parser, Subcommand};
use colored::Colorize;
use models::Priority;
use models::Status;

#[derive(Parser)]
#[command(name = "todo")]
#[command(author = "nezerkill")]
#[command(version = "0.1.0")]
#[command(about = "Terminal To-Do Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Добавить новую задачу
    Add {
        /// Текст задачи
        #[arg(required = true)]
        title: String,

        /// Проект
        #[arg(short, long)]
        project: Option<String>,

        /// Приоритет (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: Priority,

        /// Дедлайн (YYYY-MM-DD или DD.MM.YYYY)
        #[arg(short, long)]
        due: Option<String>,

        /// Теги
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Показать список задач
    List {
        /// Фильтр по проекту
        #[arg(short, long)]
        project: Option<String>,

        /// Фильтр по приоритету
        #[arg(short, long)]
        priority: Option<Priority>,

        /// Фильтр по статусу
        #[arg(short, long)]
        status: Option<Status>,

        /// Только просроченные
        #[arg(long)]
        overdue: bool,

        /// Только ожидающие
        #[arg(long)]
        pending: bool,

        /// Только выполненные
        #[arg(long)]
        done: bool,
    },

    /// Отметить задачу выполненной
    Done {
        /// ID задачи (можно кратко, первые 8 символов)
        id: String,
    },

    /// Удалить задачу
    Remove {
        /// ID задачи (можно кратко, первые 8 символов)
        id: String,
    },

    /// Показать статистику
    Stats,

    /// TUI интерфейс (vim-like)
    Tui,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        Commands::Add {
            title,
            project,
            priority,
            due,
            tags,
        } => commands::add::execute(title, project, priority, due, tags),

        Commands::List {
            project,
            priority,
            status,
            overdue,
            pending,
            done,
        } => commands::list::execute(project, priority, status, overdue, pending, done),

        Commands::Done { id } => commands::done::execute(id),

        Commands::Remove { id } => commands::remove::execute(id),

        Commands::Stats => commands::stats::execute(),

        Commands::Tui => commands::tui::execute(),
    } {
        eprintln!("{} {}", "Ошибка:".red().bold(), e);
        std::process::exit(1);
    }
}
