//! HotkeyFooter Demo
//!
//! Shows a hotkey footer with different style presets.
//! Press 's' to cycle styles, 'b' to toggle brackets.
//!
//! Run with: cargo run --example hotkey_footer_demo

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use ratatui_interact::components::{HotkeyFooter, HotkeyFooterStyle, HotkeyItem};
use ratatui_interact::events::is_close_key;

struct App {
    style_index: usize,
    should_quit: bool,
}

const STYLE_NAMES: &[&str] = &["Default", "Minimal", "Vim", "Centered", "Pipe-separated"];

impl App {
    fn new() -> Self {
        Self {
            style_index: 0,
            should_quit: false,
        }
    }

    fn cycle_style(&mut self) {
        self.style_index = (self.style_index + 1) % STYLE_NAMES.len();
    }

    fn current_style(&self) -> HotkeyFooterStyle {
        match self.style_index {
            0 => HotkeyFooterStyle::default(),
            1 => HotkeyFooterStyle::minimal(),
            2 => HotkeyFooterStyle::vim(),
            3 => HotkeyFooterStyle::default().alignment(Alignment::Center),
            4 => HotkeyFooterStyle::default()
                .separator(" | ")
                .bracket_key(false),
            _ => HotkeyFooterStyle::default(),
        }
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
                } else if key.code == KeyCode::Char('s') {
                    app.cycle_style();
                }
            }
        }

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
            Constraint::Length(3),  // Title
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("HotkeyFooter Demo")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Demo "));
    f.render_widget(title, chunks[0]);

    // Content
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  s", Style::default().fg(Color::Yellow)),
            Span::raw(": Cycle style preset    "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
        Line::from(""),
        Line::from(format!(
            "  Current style: {}",
            STYLE_NAMES[app.style_index]
        )),
    ]);
    f.render_widget(content, chunks[1]);

    // Hotkey footer
    let items = vec![
        HotkeyItem::new("q", "Quit"),
        HotkeyItem::new("s", "Style"),
        HotkeyItem::new("?", "Help"),
        HotkeyItem::new("Ctrl+C", "Force quit"),
    ];

    let footer = HotkeyFooter::new(&items).style(app.current_style());
    f.render_widget(footer, chunks[2]);
}
