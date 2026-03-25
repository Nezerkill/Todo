use crate::models::Priority;
use crate::tui::app::{App, InputMode};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

pub fn run(mut app: App) -> Result<()> {
    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    let res = run_app(&mut terminal, &mut app);

    terminal.show_cursor()?;
    terminal.clear()?;

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
        terminal.draw(|f| super::ui::draw(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
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
        KeyCode::Char('d') => {
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
            app.set_message("✓ Обновлено");
        }
        KeyCode::Char('A') => {
            // Быстрое добавление с высоким приоритетом
            app.mode = InputMode::Add;
            app.input.clear();
            app.cursor = 0;
        }
        KeyCode::Esc => {
            app.filter = Default::default();
            app.set_message("✓ Фильтры сброшены");
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
            app.input.insert(app.cursor, c);
            app.cursor += 1;
        }
        KeyCode::Backspace => {
            if app.cursor > 0 {
                app.input.remove(app.cursor - 1);
                app.cursor -= 1;
            }
        }
        KeyCode::Delete => {
            if app.cursor < app.input.len() {
                app.input.remove(app.cursor);
            }
        }
        KeyCode::Left => {
            if app.cursor > 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor < app.input.len() {
                app.cursor += 1;
            }
        }
        KeyCode::Home => {
            app.cursor = 0;
        }
        KeyCode::End => {
            app.cursor = app.input.len();
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
            app.input.insert(app.cursor, c);
            app.cursor += 1;
        }
        KeyCode::Backspace => {
            if app.cursor > 0 {
                app.input.remove(app.cursor - 1);
                app.cursor -= 1;
            }
        }
        KeyCode::Left => {
            if app.cursor > 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor < app.input.len() {
                app.cursor += 1;
            }
        }
        _ => {}
    }
}
