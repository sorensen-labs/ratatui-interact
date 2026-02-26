//! StatusLine Demo
//!
//! Interactive demo showing StatusLine features:
//! - Left, center, and right sections
//! - PowerLine-style separators
//! - Multiple presets (editor, git, minimal, powerline)
//! - Cycling through configurations with Left/Right
//!
//! Run with: cargo run --example status_line_demo

use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use ratatui_interact::components::status_line::powerline;
use ratatui_interact::components::{StatusLine, StatusLineStyle};
use ratatui_interact::events::is_close_key;

/// Preset configurations to cycle through
#[derive(Clone, Copy, PartialEq)]
enum Preset {
    Editor,
    Git,
    Minimal,
    Powerline,
}

impl Preset {
    const ALL: &[Preset] = &[
        Preset::Editor,
        Preset::Git,
        Preset::Minimal,
        Preset::Powerline,
    ];

    fn name(self) -> &'static str {
        match self {
            Preset::Editor => "Editor",
            Preset::Git => "Git Status",
            Preset::Minimal => "Minimal",
            Preset::Powerline => "PowerLine",
        }
    }
}

struct App {
    preset_idx: usize,
    mode: &'static str,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            preset_idx: 0,
            mode: "NORMAL",
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

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            "NORMAL" => "INSERT",
            "INSERT" => "VISUAL",
            _ => "NORMAL",
        };
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
            } else if key.code == KeyCode::Char('m') {
                app.toggle_mode();
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
            Constraint::Min(1),   // Description
            Constraint::Length(1), // Status line
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Title
    let title = Paragraph::new("StatusLine Demo")
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

    // Description
    let desc = match app.preset() {
        Preset::Editor => vec![
            Line::from(Span::styled(
                "Editor-style status bar",
                Style::default().fg(Color::White),
            )),
            Line::from(Span::styled(
                "Shows mode, filename, and cursor position",
                Style::default().fg(Color::DarkGray),
            )),
        ],
        Preset::Git => vec![
            Line::from(Span::styled(
                "Git-style status bar",
                Style::default().fg(Color::White),
            )),
            Line::from(Span::styled(
                "Shows branch, repo name, and file stats",
                Style::default().fg(Color::DarkGray),
            )),
        ],
        Preset::Minimal => vec![
            Line::from(Span::styled(
                "Minimal status bar",
                Style::default().fg(Color::White),
            )),
            Line::from(Span::styled(
                "Simple left/right layout, no separators",
                Style::default().fg(Color::DarkGray),
            )),
        ],
        Preset::Powerline => vec![
            Line::from(Span::styled(
                "PowerLine-style status bar (requires Nerd Font)",
                Style::default().fg(Color::White),
            )),
            Line::from(Span::styled(
                "Uses arrow separators between colored sections",
                Style::default().fg(Color::DarkGray),
            )),
        ],
    };
    f.render_widget(Paragraph::new(desc), chunks[2]);

    // Render the actual status line
    let status_line = build_status_line(app);
    status_line.render(chunks[3], f.buffer_mut());

    // Help
    let help_lines = vec![Line::from(vec![
        Span::styled("Left/Right", Style::default().fg(Color::Yellow)),
        Span::raw(": Change preset  "),
        Span::styled("m", Style::default().fg(Color::Yellow)),
        Span::raw(": Toggle mode  "),
        Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit"),
    ])];
    let help = Paragraph::new(help_lines).block(Block::default().borders(Borders::TOP));
    f.render_widget(help, chunks[4]);
}

fn build_status_line<'a>(app: &'a App) -> StatusLine<'a> {
    match app.preset() {
        Preset::Editor => {
            let mode_style = match app.mode {
                "INSERT" => Style::default().fg(Color::Black).bg(Color::Green),
                "VISUAL" => Style::default().fg(Color::Black).bg(Color::Magenta),
                _ => Style::default().fg(Color::Black).bg(Color::Blue),
            };

            StatusLine::new()
                .left_section(Line::from(Span::styled(
                    format!(" {} ", app.mode),
                    mode_style.add_modifier(Modifier::BOLD),
                )))
                .left_section(Line::from(Span::styled(
                    " main.rs ",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                )))
                .center(Line::from(Span::styled(
                    "[modified]",
                    Style::default().fg(Color::Yellow),
                )))
                .right_section(Line::from(Span::styled(
                    " Ln 42, Col 8 ",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                )))
                .right_section(Line::from(Span::styled(
                    " UTF-8 ",
                    Style::default().fg(Color::Gray),
                )))
                .style(StatusLineStyle::default().background(
                    Style::default().bg(Color::Rgb(30, 30, 30)),
                ))
        }
        Preset::Git => StatusLine::new()
            .left_section(Line::from(Span::styled(
                "  main ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )))
            .left_section(Line::from(Span::styled(
                " ratatui-interact ",
                Style::default().fg(Color::White).bg(Color::DarkGray),
            )))
            .center(Line::from(Span::styled(
                "3 staged | 1 modified",
                Style::default().fg(Color::Yellow),
            )))
            .right_section(Line::from(Span::styled(
                " +142 -37 ",
                Style::default().fg(Color::White).bg(Color::DarkGray),
            )))
            .right_section(Line::from(Span::styled(
                " 24 files ",
                Style::default().fg(Color::Gray),
            )))
            .style(
                StatusLineStyle::default()
                    .background(Style::default().bg(Color::Rgb(30, 30, 30))),
            ),
        Preset::Minimal => StatusLine::new()
            .left_section(Line::from(Span::styled(
                app.mode,
                Style::default().fg(Color::Cyan),
            )))
            .right_section(Line::from(Span::styled(
                "42:8",
                Style::default().fg(Color::Gray),
            ))),
        Preset::Powerline => {
            let mode_style = match app.mode {
                "INSERT" => Style::default().fg(Color::Black).bg(Color::Green),
                "VISUAL" => Style::default().fg(Color::Black).bg(Color::Magenta),
                _ => Style::default().fg(Color::Black).bg(Color::Blue),
            };

            let sep_style = match app.mode {
                "INSERT" => Style::default().fg(Color::Green).bg(Color::DarkGray),
                "VISUAL" => Style::default().fg(Color::Magenta).bg(Color::DarkGray),
                _ => Style::default().fg(Color::Blue).bg(Color::DarkGray),
            };

            StatusLine::new()
                .left_section_with_sep(
                    Line::from(Span::styled(
                        format!(" {} ", app.mode),
                        mode_style.add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(powerline::ARROW_RIGHT, sep_style)),
                )
                .left_section_with_sep(
                    Line::from(Span::styled(
                        " main.rs ",
                        Style::default().fg(Color::White).bg(Color::DarkGray),
                    )),
                    Line::from(Span::styled(
                        powerline::ARROW_RIGHT,
                        Style::default()
                            .fg(Color::DarkGray)
                            .bg(Color::Rgb(30, 30, 30)),
                    )),
                )
                .center(Line::from(Span::styled(
                    "[modified]",
                    Style::default().fg(Color::Yellow),
                )))
                .right_section_with_sep(
                    Line::from(Span::styled(
                        " Ln 42 ",
                        Style::default().fg(Color::White).bg(Color::DarkGray),
                    )),
                    Line::from(Span::styled(
                        powerline::ARROW_LEFT,
                        Style::default()
                            .fg(Color::DarkGray)
                            .bg(Color::Rgb(30, 30, 30)),
                    )),
                )
                .right_section_with_sep(
                    Line::from(Span::styled(
                        " Col 8 ",
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        powerline::ARROW_LEFT,
                        Style::default().fg(Color::Cyan).bg(Color::DarkGray),
                    )),
                )
                .style(StatusLineStyle::default().background(
                    Style::default().bg(Color::Rgb(30, 30, 30)),
                ))
        }
    }
}
