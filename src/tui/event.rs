use crate::models::{Priority, Status};
use crate::tui::app::{App, Filter, InputMode};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

pub fn run(mut app: App) -> Result<()> {
    // Включаем raw mode и альтернативный экран
    enable_raw_mode()?;
    let mut stdout = io::stderr();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Запускаем приложение
    let res = run_app(&mut terminal, &mut app);

    // Восстанавливаем терминал
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Ошибка: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    app: &mut App,
) -> Result<()> {
    loop {
        // Принудительная перерисовка всего экрана
        terminal.draw(|f| super::ui::draw(f, app))?;

        // Ожидание события с таймаутом
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Обрабатываем только нажатия (не повторения)
                if key.kind == KeyEventKind::Press {
                    match app.mode {
                        InputMode::Normal => {
                            if handle_normal_mode(key, app)? {
                                return Ok(());
                            }
                        }
                        InputMode::Add => handle_add_mode(key, app),
                        InputMode::Search => handle_search_mode(key, app),
                    }
                }
            }
        }

        // Обновляем таймеры (сообщения)
        app.tick();
    }
}

fn handle_normal_mode(key: KeyEvent, app: &mut App) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => {
            return Ok(true);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.select_previous();
        }
        KeyCode::Char('g') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.select_last();
            } else {
                app.select_first();
            }
        }
        KeyCode::Char('G') => {
            app.select_last();
        }
        KeyCode::Enter => {
            app.mark_done()?;
        }
        KeyCode::Char('d') | KeyCode::Char('x') => {
            app.delete_task()?;
        }
        KeyCode::Char('a') => {
            app.mode = InputMode::Add;
            app.input.clear();
            app.cursor = 0;
        }
        KeyCode::Char('/') => {
            app.mode = InputMode::Search;
            app.input.clear();
            app.cursor = 0;
        }
        KeyCode::Char('r') => {
            app.reload()?;
            app.set_message("Reloaded");
        }
        KeyCode::Char('A') => {
            // Быстрое добавление с высоким приоритетом
            app.mode = InputMode::Add;
            app.input.clear();
            app.cursor = 0;
        }
        KeyCode::Esc => {
            app.filter = Filter::default();
            app.clamp_selected();
            app.set_message("Filters reset");
        }
        KeyCode::Char('p') => {
            // Переключение фильтра по статусу (все/pending)
            if app.filter.status.is_some() {
                app.set_filter_status(None);
                app.set_message("Showing all tasks");
            } else {
                app.set_filter_status(Some(Status::Pending));
                app.set_message("Showing pending only");
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_add_mode(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let input = app.input.clone();

                // Парсинг: "задача #проект !high"
                let mut title = input.clone();
                let mut priority = Priority::Medium;
                let mut project = None;

                // Приоритет
                if let Some(idx) = input.rfind("!high") {
                    priority = Priority::High;
                    title.replace_range(idx..idx + 5, "");
                } else if let Some(idx) = input.rfind("!low") {
                    priority = Priority::Low;
                    title.replace_range(idx..idx + 4, "");
                }

                // Проект
                if let Some(idx) = input.rfind('#') {
                    if let Some(space_idx) = input[idx..].find(' ') {
                        project = Some(input[idx + 1..idx + space_idx].to_string());
                        title.replace_range(idx..idx + space_idx + 1, "");
                    } else {
                        project = Some(input[idx + 1..].to_string());
                        title.replace_range(idx.., "");
                    }
                }

                // Очищаем input ДО добавления задачи
                app.input.clear();
                app.cursor = 0;
                app.mode = InputMode::Normal;

                let _ = app.add_task(title.trim().to_string(), priority, project);
            } else {
                app.mode = InputMode::Normal;
                app.input.clear();
                app.cursor = 0;
            }
        }
        KeyCode::Esc => {
            app.mode = InputMode::Normal;
            app.input.clear();
            app.cursor = 0;
        }
        KeyCode::Char(c) => {
            // Вставляем символ по индексу в символах, а не байтах
            let mut char_idx = 0;
            let mut byte_idx = 0;
            for (i, ch) in app.input.char_indices() {
                if char_idx == app.cursor {
                    byte_idx = i;
                    break;
                }
                char_idx += 1;
                byte_idx = i + ch.len_utf8();
            }
            app.input.insert_str(byte_idx, &c.to_string());
            app.cursor += 1;
        }
        KeyCode::Backspace => {
            if app.cursor > 0 && !app.input.is_empty() {
                // Находим позицию предыдущего символа
                let mut char_indices: Vec<(usize, char)> = app.input.char_indices().collect();
                if app.cursor <= char_indices.len() {
                    if app.cursor > 0 {
                        let remove_idx = if app.cursor == char_indices.len() {
                            char_indices.last().map(|(i, _)| *i).unwrap_or(0)
                        } else {
                            char_indices[app.cursor - 1].0
                        };
                        let char_len = app.input[remove_idx..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                        app.input.replace_range(remove_idx..remove_idx + char_len, "");
                        app.cursor -= 1;
                    }
                }
            }
        }
        KeyCode::Delete => {
            if app.cursor < app.input.chars().count() && !app.input.is_empty() {
                let char_indices: Vec<(usize, char)> = app.input.char_indices().collect();
                if app.cursor < char_indices.len() {
                    let remove_idx = char_indices[app.cursor].0;
                    let char_len = char_indices[app.cursor].1.len_utf8();
                    app.input.replace_range(remove_idx..remove_idx + char_len, "");
                }
            }
        }
        KeyCode::Left => {
            if app.cursor > 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor < app.input.chars().count() {
                app.cursor += 1;
            }
        }
        KeyCode::Home => {
            app.cursor = 0;
        }
        KeyCode::End => {
            app.cursor = app.input.chars().count();
        }
        _ => {}
    }
}

fn handle_search_mode(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if app.input.is_empty() {
                app.filter.search = None;
            } else {
                app.filter.search = Some(app.input.clone());
            }
            app.mode = InputMode::Normal;
            app.input.clear();
            app.cursor = 0;
            app.select_first();
        }
        KeyCode::Esc => {
            app.mode = InputMode::Normal;
            app.input.clear();
            app.cursor = 0;
            app.filter.search = None;
        }
        KeyCode::Char(c) => {
            let mut char_idx = 0;
            let mut byte_idx = 0;
            for (i, ch) in app.input.char_indices() {
                if char_idx == app.cursor {
                    byte_idx = i;
                    break;
                }
                char_idx += 1;
                byte_idx = i + ch.len_utf8();
            }
            app.input.insert_str(byte_idx, &c.to_string());
            app.cursor += 1;
        }
        KeyCode::Backspace => {
            if app.cursor > 0 && !app.input.is_empty() {
                let char_indices: Vec<(usize, char)> = app.input.char_indices().collect();
                if app.cursor <= char_indices.len() && app.cursor > 0 {
                    let remove_idx = if app.cursor == char_indices.len() {
                        char_indices.last().map(|(i, _)| *i).unwrap_or(0)
                    } else {
                        char_indices[app.cursor - 1].0
                    };
                    let char_len = app.input[remove_idx..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                    app.input.replace_range(remove_idx..remove_idx + char_len, "");
                    app.cursor -= 1;
                }
            }
        }
        KeyCode::Left => {
            if app.cursor > 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor < app.input.chars().count() {
                app.cursor += 1;
            }
        }
        KeyCode::Home => {
            app.cursor = 0;
        }
        KeyCode::End => {
            app.cursor = app.input.chars().count();
        }
        _ => {}
    }
}
