//! StatusLine Demo
//!
//! Shows a status line with left, center, and right sections.
//! Toggle PowerLine separators with 's', cycle modes with 'm'.
//!
//! Run with: cargo run --example status_line_demo

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use ratatui_interact::components::status_line::powerline;
use ratatui_interact::components::{StatusLine, StatusLineStyle};
use ratatui_interact::events::is_close_key;

struct App {
    mode: &'static str,
    use_powerline: bool,
    counter: usize,
    should_quit: bool,
}

const MODES: &[&str] = &["NORMAL", "INSERT", "VISUAL", "COMMAND"];

impl App {
    fn new() -> Self {
        Self {
            mode: MODES[0],
            use_powerline: true,
            counter: 0,
            should_quit: false,
        }
    }

    fn cycle_mode(&mut self) {
        let idx = MODES.iter().position(|m| *m == self.mode).unwrap_or(0);
        self.mode = MODES[(idx + 1) % MODES.len()];
    }

    fn toggle_powerline(&mut self) {
        self.use_powerline = !self.use_powerline;
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if is_close_key(&key) || key.code == KeyCode::Char('q') {
                    app.should_quit = true;
                } else if key.code == KeyCode::Char('m') {
                    app.cycle_mode();
                } else if key.code == KeyCode::Char('s') {
                    app.toggle_powerline();
                }
            }
        }

        app.counter += 1;

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Status line
        ])
        .split(area);

    // Title
    let title = Paragraph::new("StatusLine Demo")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Demo "));
    f.render_widget(title, chunks[0]);

    // Content
    let hint_style = if app.use_powerline {
        "PowerLine ON"
    } else {
        "PowerLine OFF"
    };
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  m", Style::default().fg(Color::Yellow)),
            Span::raw(": Cycle mode    "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle PowerLine separators    "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
        Line::from(""),
        Line::from(format!("  Mode: {}  |  Style: {}", app.mode, hint_style)),
        Line::from(format!("  Frame: {}", app.counter)),
    ]);
    f.render_widget(content, chunks[1]);

    // Status line
    let mode_color = match app.mode {
        "NORMAL" => Color::Green,
        "INSERT" => Color::Blue,
        "VISUAL" => Color::Magenta,
        "COMMAND" => Color::Yellow,
        _ => Color::White,
    };

    let bg_color = Color::Rgb(40, 40, 40);
    let style = StatusLineStyle::default()
        .background(Style::default().bg(bg_color).fg(Color::White))
        .center_margin(1);

    let status = if app.use_powerline {
        StatusLine::new()
            .left_section_with_sep(
                Span::from(format!(" {} ", app.mode))
                    .style(Style::default().fg(Color::Black).bg(mode_color)),
                Span::from(powerline::SLANT_RIGHT)
                    .style(Style::default().fg(mode_color).bg(Color::DarkGray)),
            )
            .left_section_with_sep(
                Span::from(" main ").style(Style::default().fg(Color::White).bg(Color::DarkGray)),
                Span::from(powerline::SLANT_RIGHT)
                    .style(Style::default().fg(Color::DarkGray).bg(bg_color)),
            )
            .center(Line::from("status_line_demo.rs"))
            .right_section_with_sep(
                Span::from(" Ln 42, Col 8 ".to_string())
                    .style(Style::default().fg(Color::White).bg(Color::DarkGray)),
                Span::from(powerline::SLANT_LEFT)
                    .style(Style::default().fg(Color::DarkGray).bg(bg_color)),
            )
            .right_section_with_sep(
                Span::from(" UTF-8 ").style(Style::default().fg(Color::Black).bg(Color::Cyan)),
                Span::from(powerline::SLANT_LEFT)
                    .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray)),
            )
            .style(style)
    } else {
        StatusLine::new()
            .left_section(
                Span::from(format!(" {} ", app.mode))
                    .style(Style::default().fg(Color::Black).bg(mode_color)),
            )
            .left_section(
                Span::from(" main ").style(Style::default().fg(Color::White).bg(Color::DarkGray)),
            )
            .center(Line::from("status_line_demo.rs"))
            .right_section(
                Span::from(" Ln 42, Col 8 ")
                    .style(Style::default().fg(Color::White).bg(Color::DarkGray)),
            )
            .right_section(
                Span::from(" UTF-8 ").style(Style::default().fg(Color::Black).bg(Color::Cyan)),
            )
            .style(style)
    };

    f.render_widget(status, chunks[2]);
}
