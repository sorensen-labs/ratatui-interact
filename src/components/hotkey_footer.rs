//! HotkeyFooter widget — single-line hotkey hint display.
//!
//! Renders a row of key/description pairs, styled with configurable colors,
//! optional bracket wrapping, and alignment.
//!
//! # Example
//!
//! ```rust
//! use ratatui_interact::components::{HotkeyFooter, HotkeyFooterStyle, HotkeyItem};
//! use ratatui::layout::Rect;
//! use ratatui::buffer::Buffer;
//! use ratatui::widgets::Widget;
//!
//! let items = vec![
//!     HotkeyItem::new("q", "Quit"),
//!     HotkeyItem::new("?", "Help"),
//! ];
//!
//! // Default style (bold cyan keys, dark gray descriptions, brackets)
//! let footer = HotkeyFooter::new(&items);
//!
//! // Minimal preset (no brackets, white keys)
//! let footer = HotkeyFooter::new(&items)
//!     .style(HotkeyFooterStyle::minimal());
//!
//! // Vim preset
//! let footer = HotkeyFooter::new(&items)
//!     .style(HotkeyFooterStyle::vim());
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// A key-description pair for hotkey display.
#[derive(Debug, Clone)]
pub struct HotkeyItem {
    /// The key label (e.g. "q", "?", "Ctrl+C").
    pub key: String,
    /// The human-readable description (e.g. "Quit", "Help").
    pub description: String,
}

impl HotkeyItem {
    /// Create a new hotkey item.
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// Style configuration for the hotkey footer.
#[derive(Debug, Clone)]
pub struct HotkeyFooterStyle {
    /// Style for the key text (default: bold cyan).
    pub key_style: Style,
    /// Style for the description text (default: dark gray).
    pub description_style: Style,
    /// String rendered between key-description pairs (default: two spaces).
    pub separator: String,
    /// Background style applied to the entire footer row (default: black bg).
    pub background: Style,
    /// Whether to wrap key labels in brackets like `[q]` (default: true).
    pub bracket_key: bool,
    /// Horizontal alignment of the hotkey row (default: Left).
    pub alignment: Alignment,
}

impl Default for HotkeyFooterStyle {
    fn default() -> Self {
        Self {
            key_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            description_style: Style::default().fg(Color::DarkGray),
            separator: "  ".to_string(),
            background: Style::default().bg(Color::Black),
            bracket_key: true,
            alignment: Alignment::Left,
        }
    }
}

impl HotkeyFooterStyle {
    /// Minimal preset: no brackets, white keys, gray descriptions.
    pub fn minimal() -> Self {
        Self {
            key_style: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            description_style: Style::default().fg(Color::DarkGray),
            separator: "  ".to_string(),
            background: Style::default().bg(Color::Black),
            bracket_key: false,
            alignment: Alignment::Left,
        }
    }

    /// Vim preset: no brackets, green keys, white descriptions.
    pub fn vim() -> Self {
        Self {
            key_style: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            description_style: Style::default().fg(Color::White),
            separator: "  ".to_string(),
            background: Style::default().bg(Color::Black),
            bracket_key: false,
            alignment: Alignment::Left,
        }
    }

    /// Set the key style.
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Set the description style.
    pub fn description_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }

    /// Set the separator string rendered between pairs.
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    /// Set the background style for the entire footer row.
    pub fn background(mut self, style: Style) -> Self {
        self.background = style;
        self
    }

    /// Set whether to wrap keys in brackets.
    pub fn bracket_key(mut self, bracket: bool) -> Self {
        self.bracket_key = bracket;
        self
    }

    /// Set the horizontal alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

/// A single-line hotkey hint display widget.
///
/// Renders key/description pairs in a horizontal row with configurable styling.
#[derive(Debug, Clone)]
pub struct HotkeyFooter<'a> {
    items: &'a [HotkeyItem],
    style: HotkeyFooterStyle,
}

impl<'a> HotkeyFooter<'a> {
    /// Create a new footer widget from a slice of hotkey items.
    pub fn new(items: &'a [HotkeyItem]) -> Self {
        Self {
            items,
            style: HotkeyFooterStyle::default(),
        }
    }

    /// Set the style configuration.
    pub fn style(mut self, style: HotkeyFooterStyle) -> Self {
        self.style = style;
        self
    }

    /// Build the span list for all items.
    fn build_spans(&self) -> Vec<Span<'static>> {
        let mut spans: Vec<Span<'static>> = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(
                    self.style.separator.clone(),
                    self.style.description_style,
                ));
            }

            if self.style.bracket_key {
                spans.push(Span::styled(
                    format!("[{}]", item.key),
                    self.style.key_style,
                ));
            } else {
                spans.push(Span::styled(item.key.clone(), self.style.key_style));
            }

            spans.push(Span::styled(
                format!(" {}", item.description),
                self.style.description_style,
            ));
        }

        spans
    }
}

impl Widget for HotkeyFooter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let spans = self.build_spans();
        let line = Line::from(spans);
        Paragraph::new(line)
            .style(self.style.background)
            .alignment(self.style.alignment)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_item_new() {
        let item = HotkeyItem::new("q", "Quit");
        assert_eq!(item.key, "q");
        assert_eq!(item.description, "Quit");

        // Verify Into<String> works with owned strings
        let item2 = HotkeyItem::new(String::from("Ctrl+C"), String::from("Copy"));
        assert_eq!(item2.key, "Ctrl+C");
        assert_eq!(item2.description, "Copy");
    }

    #[test]
    fn test_default_style() {
        let style = HotkeyFooterStyle::default();
        assert_eq!(style.separator, "  ");
        assert!(style.bracket_key);
        assert_eq!(style.alignment, Alignment::Left);
        // Check colors are set (non-default plain Style)
        assert_ne!(style.key_style, Style::default());
        assert_ne!(style.description_style, Style::default());
    }

    #[test]
    fn test_style_presets() {
        let minimal = HotkeyFooterStyle::minimal();
        let vim = HotkeyFooterStyle::vim();

        // Both presets disable brackets
        assert!(!minimal.bracket_key);
        assert!(!vim.bracket_key);

        // Presets have different key colors from each other
        assert_ne!(minimal.key_style, vim.key_style);

        // Presets differ from default
        let default = HotkeyFooterStyle::default();
        assert!(default.bracket_key);
        assert!(!minimal.bracket_key);
    }

    #[test]
    fn test_footer_new() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let footer = HotkeyFooter::new(&items);
        assert_eq!(footer.items.len(), 1);
    }

    #[test]
    fn test_style_builder() {
        let style = HotkeyFooterStyle::default()
            .bracket_key(false)
            .separator(" | ")
            .alignment(Alignment::Center)
            .key_style(Style::default().fg(Color::Red))
            .description_style(Style::default().fg(Color::White))
            .background(Style::default().bg(Color::Blue));

        assert!(!style.bracket_key);
        assert_eq!(style.separator, " | ");
        assert_eq!(style.alignment, Alignment::Center);
        assert_eq!(style.key_style, Style::default().fg(Color::Red));
        assert_eq!(style.description_style, Style::default().fg(Color::White));
        assert_eq!(style.background, Style::default().bg(Color::Blue));
    }

    #[test]
    fn test_render_empty_items() {
        let items: Vec<HotkeyItem> = vec![];
        let footer = HotkeyFooter::new(&items);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));

        // Must not panic
        footer.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    #[test]
    fn test_render_with_items() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let footer = HotkeyFooter::new(&items);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));

        // Must not panic
        footer.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    #[test]
    fn test_render_zero_area() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let footer = HotkeyFooter::new(&items);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));

        // Zero width — must not panic
        footer.render(Rect::new(0, 0, 0, 1), &mut buf);

        // Zero height — must not panic
        let footer2 = HotkeyFooter::new(&items);
        footer2.render(Rect::new(0, 0, 40, 0), &mut buf);
    }

    #[test]
    fn test_bracket_key_formatting() {
        let items = vec![HotkeyItem::new("q", "Quit")];

        // With brackets
        let footer_bracketed = HotkeyFooter::new(&items);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        footer_bracketed.render(Rect::new(0, 0, 40, 1), &mut buf);
        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains('['), "bracketed mode should render '['");
        assert!(content.contains(']'), "bracketed mode should render ']'");

        // Without brackets
        let style = HotkeyFooterStyle::default().bracket_key(false);
        let footer_plain = HotkeyFooter::new(&items).style(style);
        let mut buf2 = Buffer::empty(Rect::new(0, 0, 40, 1));
        footer_plain.render(Rect::new(0, 0, 40, 1), &mut buf2);
        let content2: String = buf2
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(!content2.contains('['), "plain mode should not render '['");
    }

    // -------------------------------------------------------------------------
    // ADVERSARIAL TESTS — fromage-press
    // -------------------------------------------------------------------------

    // --- Empty / extreme item counts ---

    /// Empty items slice — must not panic, buffer should be pristine background.
    #[test]
    fn test_empty_items_no_output() {
        let items: Vec<HotkeyItem> = vec![];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
        // No key brackets should appear.
        let content: String = buf.content.iter().map(|c| c.symbol().to_string()).collect();
        assert!(!content.contains('['), "empty items should not produce '[' chars");
    }

    /// 100 items — must not panic.
    #[test]
    fn test_hundred_items_no_panic() {
        let items: Vec<HotkeyItem> = (0..100)
            .map(|i| HotkeyItem::new(format!("k{i}"), format!("Action {i}")))
            .collect();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// Item with empty key string — bracketed rendering produces "[]", must not panic.
    #[test]
    fn test_empty_key_string_bracketed() {
        let items = vec![HotkeyItem::new("", "Action")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Item with empty key, no brackets — produces empty span, must not panic.
    #[test]
    fn test_empty_key_string_plain() {
        let items = vec![HotkeyItem::new("", "Action")];
        let style = HotkeyFooterStyle::default().bracket_key(false);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Item with empty description — produces " " (space + empty), must not panic.
    #[test]
    fn test_empty_description_string() {
        let items = vec![HotkeyItem::new("q", "")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Both key and description empty — must not panic.
    #[test]
    fn test_fully_empty_item() {
        let items = vec![HotkeyItem::new("", "")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    // --- Unicode in keys and descriptions ---

    /// Emoji key — must not panic.
    #[test]
    fn test_emoji_key() {
        let items = vec![HotkeyItem::new("🧀", "Cheese mode")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// CJK characters in key — wide chars must not panic.
    #[test]
    fn test_cjk_key() {
        let items = vec![HotkeyItem::new("中", "Chinese key")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// CJK characters in description — must not panic.
    #[test]
    fn test_cjk_description() {
        let items = vec![HotkeyItem::new("q", "退出")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// RTL text (Arabic) in description — must not panic.
    #[test]
    fn test_rtl_description() {
        let items = vec![HotkeyItem::new("q", "خروج")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Zero-width joiner in key — must not panic.
    #[test]
    fn test_zero_width_joiner_key() {
        let items = vec![HotkeyItem::new("A\u{200D}B", "ZWJ key")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    // --- Separator edge cases ---

    /// Empty separator string — must not panic, no separator gap rendered.
    #[test]
    fn test_empty_separator() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let style = HotkeyFooterStyle::default().separator("");
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Very long separator — must not panic.
    #[test]
    fn test_very_long_separator() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let long_sep = " | ".repeat(200);
        let style = HotkeyFooterStyle::default().separator(long_sep);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Separator containing a newline — must not panic (Paragraph handles it).
    #[test]
    fn test_newline_separator() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let style = HotkeyFooterStyle::default().separator("\n");
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Separator with tab character — must not panic.
    #[test]
    fn test_tab_separator() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let style = HotkeyFooterStyle::default().separator("\t");
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Unicode separator (powerline arrow) — must not panic.
    #[test]
    fn test_unicode_separator() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
        ];
        let style = HotkeyFooterStyle::default().separator(" \u{e0b0} ");
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    // --- Tiny area attacks ---

    /// 1-char wide area with many items — must not panic.
    #[test]
    fn test_one_char_wide_area() {
        let items = vec![
            HotkeyItem::new("q", "Quit"),
            HotkeyItem::new("?", "Help"),
            HotkeyItem::new("r", "Refresh"),
        ];
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 1, 1), &mut buf);
    }

    /// Zero-width area — early return, must not panic.
    #[test]
    fn test_zero_width_area_footer() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 0, 1), &mut buf);
    }

    /// Zero-height area — early return, must not panic.
    #[test]
    fn test_zero_height_area_footer() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 10, 0), &mut buf);
    }

    // --- Style extremes ---

    /// All alignment variants render without panic.
    #[test]
    fn test_all_alignments_no_panic() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let style = HotkeyFooterStyle::default().alignment(alignment);
            let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
            HotkeyFooter::new(&items)
                .style(style)
                .render(Rect::new(0, 0, 40, 1), &mut buf);
        }
    }

    /// Style with no background color — must not panic.
    #[test]
    fn test_no_background_style() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let style = HotkeyFooterStyle::default().background(Style::default());
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).style(style).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Vim preset renders without panic and produces no brackets.
    #[test]
    fn test_vim_preset_no_brackets() {
        let items = vec![HotkeyItem::new("q", "Quit"), HotkeyItem::new("w", "Write")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items)
            .style(HotkeyFooterStyle::vim())
            .render(Rect::new(0, 0, 40, 1), &mut buf);
        let content: String = buf.content.iter().map(|c| c.symbol().to_string()).collect();
        assert!(!content.contains('['), "vim preset should not render brackets");
    }

    /// Minimal preset renders without panic.
    #[test]
    fn test_minimal_preset_renders() {
        let items = vec![HotkeyItem::new("q", "Quit")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items)
            .style(HotkeyFooterStyle::minimal())
            .render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Single item at exactly the buffer width — no separator inserted, must not panic.
    #[test]
    fn test_single_item_exact_width() {
        // Single item: no separator logic runs (i == 0 skips separator).
        let items = vec![HotkeyItem::new("q", "Quit")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Very long key — must not panic, just clipped by Paragraph.
    #[test]
    fn test_very_long_key() {
        let long_key = "K".repeat(1000);
        let items = vec![HotkeyItem::new(long_key, "Action")];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Very long description — must not panic.
    #[test]
    fn test_very_long_description() {
        let long_desc = "D".repeat(1000);
        let items = vec![HotkeyItem::new("q", long_desc)];
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        HotkeyFooter::new(&items).render(Rect::new(0, 0, 40, 1), &mut buf);
    }
}
