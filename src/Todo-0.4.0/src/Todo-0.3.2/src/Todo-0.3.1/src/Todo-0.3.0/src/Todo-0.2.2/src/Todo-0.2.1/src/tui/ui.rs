use crate::models::{Priority, Status};
use crate::tui::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    draw_title(frame, chunks[0]);
    draw_task_list(frame, chunks[1], app);
    draw_help(frame, chunks[2], app);

    if app.mode == InputMode::Add || app.mode == InputMode::Search {
        draw_input_popup(frame, app);
    }
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("📝 Todo TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(
            Style::default().fg(Color::Blue),
        ));
    frame.render_widget(title, area);
}

fn draw_task_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let filtered = app.filtered_tasks();

    let items: Vec<ListItem> = filtered
        .iter()
        .map(|task| {
            let priority_style = match task.priority {
                Priority::High => Color::Red,
                Priority::Medium => Color::Yellow,
                Priority::Low => Color::Green,
            };

            let status_icon = match task.status {
                Status::Pending => "⬜",
                Status::Done => "✅",
                Status::Deferred => "⏸️",
            };

            let priority_str = match task.priority {
                Priority::High => "🔴",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };

            let project = task
                .project
                .as_ref()
                .map(|p| format!(" [{}]", p))
                .unwrap_or_default();

            let id = &task.id[..8];
            let content = format!(
                "{} {}{} {} | {}",
                status_icon, priority_str, task.title, project, id
            );

            ListItem::new(Line::from(vec![Span::styled(
                content,
                Style::default().fg(priority_style),
            )]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(app.selected);

    let tasks_list = List::new(items)
        .block(
            Block::default()
                .title("Задачи")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(tasks_list, area, &mut list_state);
}

fn draw_help(frame: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.mode {
        InputMode::Normal => {
            "j/k: вверх/вниз | g/G: начало/конец | Enter: выполнить | d: удалить | a: добавить | /: поиск | q: выход | r: обновить"
        }
        InputMode::Add => "Enter: сохранить | Esc: отмена",
        InputMode::Search => "Enter: поиск | Esc: отмена",
    };

    let message = app
        .message
        .as_ref()
        .map(|(msg, _)| msg.as_str())
        .unwrap_or("");

    let help = Paragraph::new(format!("{} | {}", help_text, message))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Помощь")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );

    frame.render_widget(help, area);
}

fn draw_input_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.size());

    frame.render_widget(Clear, area);

    let (title, input) = match app.mode {
        InputMode::Add => ("Добавить задачу", &app.input),
        InputMode::Search => ("Поиск", &app.input),
        _ => return,
    };

    let popup = Paragraph::new(input.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(popup, area);

    frame.set_cursor(area.x + app.cursor as u16 + 1, area.y + 1);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
