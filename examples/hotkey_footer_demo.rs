//! HotkeyFooter Demo
//!
//! Interactive demo showing HotkeyFooter features:
//! - Default style with bracket-wrapped keys
//! - Minimal preset (no brackets, white keys)
//! - Vim preset (green keys, white descriptions)
//! - Custom styles (separator, alignment, colors)
//! - Cycling through presets with Left/Right
//!
//! Run with: cargo run --example hotkey_footer_demo

use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use ratatui_interact::components::{HotkeyFooter, HotkeyFooterStyle, HotkeyItem};
use ratatui_interact::events::is_close_key;

#[derive(Clone, Copy)]
enum Preset {
    Default,
    Minimal,
    Vim,
    Pipe,
    Centered,
}

impl Preset {
    const ALL: &[Preset] = &[
        Preset::Default,
        Preset::Minimal,
        Preset::Vim,
        Preset::Pipe,
        Preset::Centered,
    ];

    fn name(self) -> &'static str {
        match self {
            Preset::Default => "Default (brackets, cyan keys)",
            Preset::Minimal => "Minimal (no brackets, white keys)",
            Preset::Vim => "Vim (green keys, white desc)",
            Preset::Pipe => "Pipe separator",
            Preset::Centered => "Centered alignment",
        }
    }
}

struct App {
    preset_idx: usize,
    items: Vec<HotkeyItem>,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
            HotkeyItem::new("Tab", "Next"),
            HotkeyItem::new("Space", "Select"),
            HotkeyItem::new("Ctrl+S", "Save"),
            HotkeyItem::new("/", "Search"),
        ];
        Self {
            preset_idx: 0,
            items,
            should_quit: false,
        }
    }

    fn preset(&self) -> Preset {
        Preset::ALL[self.preset_idx]
    }

    fn next_preset(&mut self) {
        self.preset_idx = (self.preset_idx + 1) % Preset::ALL.len();
    }

    fn prev_preset(&mut self) {
        self.preset_idx = (self.preset_idx + Preset::ALL.len() - 1) % Preset::ALL.len();
    }

    fn style(&self) -> HotkeyFooterStyle {
        match self.preset() {
            Preset::Default => HotkeyFooterStyle::default(),
            Preset::Minimal => HotkeyFooterStyle::minimal(),
            Preset::Vim => HotkeyFooterStyle::vim(),
            Preset::Pipe => HotkeyFooterStyle::default()
                .separator(" | ")
                .bracket_key(false)
                .key_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            Preset::Centered => HotkeyFooterStyle::default()
                .alignment(Alignment::Center)
                .key_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .description_style(Style::default().fg(Color::White)),
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if is_close_key(&key) || key.code == KeyCode::Char('q') {
                app.should_quit = true;
            } else if key.code == KeyCode::Right || key.code == KeyCode::Char('l') {
                app.next_preset();
            } else if key.code == KeyCode::Left || key.code == KeyCode::Char('h') {
                app.prev_preset();
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Preset selector
            Constraint::Min(1),   // Description / spacer
            Constraint::Length(1), // Footer preview
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Title
    let title = Paragraph::new("HotkeyFooter Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    // Preset selector
    let preset_line = Line::from(vec![
        Span::styled("Preset: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("< {} >", app.preset().name()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ({}/{})", app.preset_idx + 1, Preset::ALL.len()),
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    f.render_widget(Paragraph::new(preset_line), chunks[1]);

    // Items list
    let item_lines: Vec<Line> = std::iter::once(Line::from(Span::styled(
        "Hotkey items:",
        Style::default().fg(Color::White),
    )))
    .chain(app.items.iter().map(|item| {
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(&item.key, Style::default().fg(Color::Cyan)),
            Span::styled(" -> ", Style::default().fg(Color::DarkGray)),
            Span::styled(&item.description, Style::default().fg(Color::Gray)),
        ])
    }))
    .collect();
    f.render_widget(Paragraph::new(item_lines), chunks[2]);

    // Render the actual footer
    let footer = HotkeyFooter::new(&app.items).style(app.style());
    footer.render(chunks[3], f.buffer_mut());

    // Help
    let help_lines = vec![Line::from(vec![
        Span::styled("Left/Right", Style::default().fg(Color::Yellow)),
        Span::raw(": Change preset  "),
        Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit"),
    ])];
    let help = Paragraph::new(help_lines).block(Block::default().borders(Borders::TOP));
    f.render_widget(help, chunks[4]);
}
