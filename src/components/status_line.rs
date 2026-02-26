//! StatusLine widget
//!
//! A single-line status bar with left, center, and right sections.
//! Supports PowerLine-style separators (requires Nerd Font).
//!
//! # Example
//!
//! ```rust
//! use ratatui_interact::components::{StatusLine, StatusLineStyle};
//! use ratatui_interact::components::status_line::powerline;
//! use ratatui::layout::Rect;
//! use ratatui::buffer::Buffer;
//! use ratatui::style::Style;
//! use ratatui::text::Line;
//! use ratatui::widgets::Widget;
//!
//! let status = StatusLine::new()
//!     .left_section(Line::from("Mode: Normal"))
//!     .left_section_with_sep(
//!         Line::from("branch: main"),
//!         Line::from(powerline::ARROW_RIGHT),
//!     )
//!     .center(Line::from("my-file.rs"))
//!     .right_section(Line::from("Ln 42, Col 8"))
//!     .style(StatusLineStyle::default());
//!
//! let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
//! status.render(Rect::new(0, 0, 80, 1), &mut buf);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::Widget,
};

/// PowerLine separator constants (require Nerd Font).
pub mod powerline {
    /// Right-pointing arrow (requires Nerd Font)
    pub const ARROW_RIGHT: &str = "\u{e0b0}";
    /// Left-pointing arrow (requires Nerd Font)
    pub const ARROW_LEFT: &str = "\u{e0b2}";
    /// Right-pointing slant (requires Nerd Font)
    pub const SLANT_RIGHT: &str = "\u{e0b8}";
    /// Left-pointing slant (requires Nerd Font)
    pub const SLANT_LEFT: &str = "\u{e0ba}";
    /// Right-pointing round (requires Nerd Font)
    pub const ROUND_RIGHT: &str = "\u{e0b4}";
    /// Left-pointing round (requires Nerd Font)
    pub const ROUND_LEFT: &str = "\u{e0b6}";
}

/// An internal section holding content and an optional separator.
#[derive(Debug, Clone)]
struct StatusLineSection<'a> {
    content: Line<'a>,
    separator: Option<Line<'a>>,
}

/// Style configuration for a [`StatusLine`].
#[derive(Debug, Clone)]
pub struct StatusLineStyle {
    /// Background fill style applied to the center region.
    pub background: Style,
    /// Padding columns around center text (default: 0).
    pub center_margin: u16,
}

impl Default for StatusLineStyle {
    fn default() -> Self {
        Self {
            background: Style::default(),
            center_margin: 0,
        }
    }
}

impl StatusLineStyle {
    /// Set the background style for the center region.
    pub fn background(mut self, style: Style) -> Self {
        self.background = style;
        self
    }

    /// Set the padding around center text.
    pub fn center_margin(mut self, margin: u16) -> Self {
        self.center_margin = margin;
        self
    }
}

/// A single-line status bar widget with left, center, and right sections.
///
/// Sections can optionally include PowerLine-style separators.
#[derive(Debug, Clone)]
pub struct StatusLine<'a> {
    left: Vec<StatusLineSection<'a>>,
    center: Option<Line<'a>>,
    right: Vec<StatusLineSection<'a>>,
    style: StatusLineStyle,
}

impl<'a> StatusLine<'a> {
    /// Create a new, empty status line.
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            center: None,
            right: Vec::new(),
            style: StatusLineStyle::default(),
        }
    }

    /// Add a left section without a separator.
    pub fn left_section(mut self, content: impl Into<Line<'a>>) -> Self {
        self.left.push(StatusLineSection {
            content: content.into(),
            separator: None,
        });
        self
    }

    /// Add a left section with a trailing separator.
    pub fn left_section_with_sep(
        mut self,
        content: impl Into<Line<'a>>,
        separator: impl Into<Line<'a>>,
    ) -> Self {
        self.left.push(StatusLineSection {
            content: content.into(),
            separator: Some(separator.into()),
        });
        self
    }

    /// Add a right section without a separator.
    pub fn right_section(mut self, content: impl Into<Line<'a>>) -> Self {
        self.right.push(StatusLineSection {
            content: content.into(),
            separator: None,
        });
        self
    }

    /// Add a right section with a leading separator (rendered to the left of content).
    pub fn right_section_with_sep(
        mut self,
        content: impl Into<Line<'a>>,
        separator: impl Into<Line<'a>>,
    ) -> Self {
        self.right.push(StatusLineSection {
            content: content.into(),
            separator: Some(separator.into()),
        });
        self
    }

    /// Set the center text.
    pub fn center(mut self, text: impl Into<Line<'a>>) -> Self {
        self.center = Some(text.into());
        self
    }

    /// Set the style configuration.
    pub fn style(mut self, style: StatusLineStyle) -> Self {
        self.style = style;
        self
    }
}

impl Default for StatusLine<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Render right sections from right to left, returning the new right edge.
///
/// Iterates in reverse so that the last-added right section renders at
/// the right edge, matching the intuitive left-to-right visual order.
fn render_right(sections: Vec<StatusLineSection<'_>>, mut x_end: u16, y: u16, buf: &mut Buffer) -> u16 {
    for section in sections.into_iter().rev() {
        let content_w = section.content.width() as u16;
        section.content.render(
            Rect::new(x_end.saturating_sub(content_w), y, content_w, 1),
            buf,
        );
        x_end = x_end.saturating_sub(content_w);

        if let Some(sep) = section.separator {
            let sep_w = sep.width() as u16;
            sep.render(Rect::new(x_end.saturating_sub(sep_w), y, sep_w, 1), buf);
            x_end = x_end.saturating_sub(sep_w);
        }
    }
    x_end
}

/// Render left sections from left to right, returning the new left edge.
fn render_left(sections: Vec<StatusLineSection<'_>>, mut x_start: u16, y: u16, buf: &mut Buffer) -> u16 {
    for section in sections {
        let content_w = section.content.width() as u16;
        section.content.render(Rect::new(x_start, y, content_w, 1), buf);
        x_start = x_start.saturating_add(content_w);

        if let Some(sep) = section.separator {
            let sep_w = sep.width() as u16;
            sep.render(Rect::new(x_start, y, sep_w, 1), buf);
            x_start = x_start.saturating_add(sep_w);
        }
    }
    x_start
}

impl Widget for StatusLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let y = area.y;
        let x_end = render_right(self.right, area.right(), y, buf);
        let x_start = render_left(self.left, area.x, y, buf);

        // CENTER FILL
        let fill_width = x_end.saturating_sub(x_start);
        if fill_width > 0 {
            buf.set_style(Rect::new(x_start, y, fill_width, 1), self.style.background);
        }

        // CENTER TEXT
        if let Some(center_line) = self.center {
            let margin = self.style.center_margin;
            let center_width = fill_width.saturating_sub(margin.saturating_mul(2));
            if center_width > 0 {
                let cx = x_start.saturating_add(margin);
                center_line.render(Rect::new(cx, y, center_width, 1), buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    #[test]
    fn test_status_line_new() {
        let status = StatusLine::new();
        assert!(status.left.is_empty());
        assert!(status.center.is_none());
        assert!(status.right.is_empty());
    }

    #[test]
    fn test_left_section_builder() {
        let status = StatusLine::new()
            .left_section(Line::from("A"))
            .left_section_with_sep(Line::from("B"), Line::from("|"));
        assert_eq!(status.left.len(), 2);
        assert!(status.left[0].separator.is_none());
        assert!(status.left[1].separator.is_some());
    }

    #[test]
    fn test_right_section_builder() {
        let status = StatusLine::new()
            .right_section(Line::from("X"))
            .right_section_with_sep(Line::from("Y"), Line::from("|"));
        assert_eq!(status.right.len(), 2);
        assert!(status.right[0].separator.is_none());
        assert!(status.right[1].separator.is_some());
    }

    #[test]
    fn test_center_builder() {
        let status = StatusLine::new().center(Line::from("center text"));
        assert!(status.center.is_some());
    }

    #[test]
    fn test_style_builder() {
        let custom_style = StatusLineStyle::default()
            .background(Style::default().fg(Color::Blue))
            .center_margin(2);
        let status = StatusLine::new().style(custom_style.clone());
        assert_eq!(status.style.center_margin, 2);
    }

    #[test]
    fn test_render_empty() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new();
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    #[test]
    fn test_render_with_sections() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new()
            .left_section(Line::from("Left"))
            .left_section_with_sep(Line::from("L2"), Line::from("|"))
            .center(Line::from("Center"))
            .right_section(Line::from("Right"))
            .right_section_with_sep(Line::from("R2"), Line::from("|"));
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    #[test]
    fn test_render_zero_area() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 1));
        let status = StatusLine::new().left_section(Line::from("text"));
        status.render(Rect::new(0, 0, 0, 1), &mut buf);

        let mut buf2 = Buffer::empty(Rect::new(0, 0, 10, 0));
        let status2 = StatusLine::new().left_section(Line::from("text"));
        status2.render(Rect::new(0, 0, 10, 0), &mut buf2);
    }

    #[test]
    fn test_default_impl() {
        let status: StatusLine = StatusLine::default();
        assert!(status.left.is_empty());
        assert!(status.center.is_none());
        assert!(status.right.is_empty());
    }

    #[test]
    fn test_powerline_constants() {
        assert!(!powerline::ARROW_RIGHT.is_empty());
        assert!(!powerline::ARROW_LEFT.is_empty());
        assert!(!powerline::SLANT_RIGHT.is_empty());
        assert!(!powerline::SLANT_LEFT.is_empty());
        assert!(!powerline::ROUND_RIGHT.is_empty());
        assert!(!powerline::ROUND_LEFT.is_empty());
    }

    #[test]
    fn test_status_line_style_default() {
        let style = StatusLineStyle::default();
        assert_eq!(style.center_margin, 0);
    }

    // -------------------------------------------------------------------------
    // ADVERSARIAL TESTS — fromage-press
    // -------------------------------------------------------------------------

    // --- Overflow chaos ---

    /// Left section content wider than the entire area — must not panic.
    #[test]
    fn test_left_overflow_wider_than_area() {
        let wide = "A".repeat(200);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let status = StatusLine::new().left_section(Line::from(wide.clone()));
        // Must not panic even though section width (200) >> area width (10).
        status.render(Rect::new(0, 0, 10, 1), &mut buf);
    }

    /// Right section content wider than the entire area — must not panic.
    #[test]
    fn test_right_overflow_wider_than_area() {
        let wide = "B".repeat(200);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let status = StatusLine::new().right_section(Line::from(wide));
        status.render(Rect::new(0, 0, 10, 1), &mut buf);
    }

    /// Left + right sections combined wider than area — center should vanish, no panic.
    #[test]
    fn test_left_right_combined_overflow() {
        let left = "L".repeat(60);
        let right = "R".repeat(60);
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new()
            .left_section(Line::from(left))
            .right_section(Line::from(right))
            .center(Line::from("SHOULD_NOT_APPEAR"));
        // 60 + 60 = 120 > 80; center_width = 0, no panic expected.
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// All three regions overflowing simultaneously — must not panic.
    #[test]
    fn test_all_three_overflow_simultaneously() {
        let left = "L".repeat(100);
        let center = "C".repeat(100);
        let right = "R".repeat(100);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .left_section(Line::from(left))
            .center(Line::from(center))
            .right_section(Line::from(right));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Left + separator combined wider than area — must not panic.
    #[test]
    fn test_left_with_sep_overflow() {
        let wide_content = "X".repeat(50);
        let wide_sep = "|".repeat(50);
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        let status =
            StatusLine::new().left_section_with_sep(Line::from(wide_content), Line::from(wide_sep));
        status.render(Rect::new(0, 0, 20, 1), &mut buf);
    }

    /// Right + separator combined wider than area — must not panic.
    #[test]
    fn test_right_with_sep_overflow() {
        let wide_content = "Y".repeat(50);
        let wide_sep = "|".repeat(50);
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        let status = StatusLine::new()
            .right_section_with_sep(Line::from(wide_content), Line::from(wide_sep));
        status.render(Rect::new(0, 0, 20, 1), &mut buf);
    }

    // --- 1x1 area attacks ---

    /// 1x1 area with full content — must not panic.
    #[test]
    fn test_one_by_one_area_full_content() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        let status = StatusLine::new()
            .left_section(Line::from("LEFTLEFTLEFT"))
            .center(Line::from("CENTERCENTER"))
            .right_section(Line::from("RIGHTRIGHTRIGHT"))
            .style(StatusLineStyle::default().center_margin(5));
        status.render(Rect::new(0, 0, 1, 1), &mut buf);
    }

    /// Zero-width area with content — must not panic (early return).
    #[test]
    fn test_zero_width_area_no_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new()
            .left_section(Line::from("Left"))
            .center(Line::from("Center"))
            .right_section(Line::from("Right"));
        status.render(Rect::new(0, 0, 0, 1), &mut buf);
    }

    /// Zero-height area — must not panic.
    #[test]
    fn test_zero_height_area_no_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new().left_section(Line::from("Left"));
        // Can only render into a zero-height buffer if buffer is non-zero height.
        // Use a 1-row buffer but render into a 0-height rect.
        status.render(Rect::new(0, 0, 80, 0), &mut buf);
    }

    // --- center_margin overflow ---

    /// center_margin so large that margin*2 overflows u16 — saturating_sub must save us.
    #[test]
    fn test_center_margin_overflow_u16() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        // margin = 40_000, margin * 2 = 80_000 which overflows u16::MAX (65535).
        let style = StatusLineStyle::default().center_margin(40_000);
        let status = StatusLine::new()
            .center(Line::from("center"))
            .style(style);
        // Should not panic; center_width should saturate to 0.
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// center_margin equal to u16::MAX — must not panic.
    #[test]
    fn test_center_margin_max_u16() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let style = StatusLineStyle::default().center_margin(u16::MAX);
        let status = StatusLine::new()
            .center(Line::from("text"))
            .style(style);
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    // --- Unicode attacks ---

    /// Multi-byte UTF-8 in left section — must not panic.
    #[test]
    fn test_unicode_multibyte_left_section() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().left_section(Line::from("こんにちは世界"));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Emoji in left section — must not panic.
    #[test]
    fn test_emoji_in_left_section() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().left_section(Line::from("🧀🔥🦀 mode"));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Emoji in center — must not panic.
    #[test]
    fn test_emoji_in_center() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().center(Line::from("🧀🧀🧀"));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Zero-width joiner sequences in content — must not panic.
    #[test]
    fn test_zero_width_joiner_content() {
        // Zero-width joiner U+200D
        let zwj = "A\u{200D}B";
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().left_section(Line::from(zwj));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Powerline separator glyphs — must not panic.
    #[test]
    fn test_powerline_separators_render() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .left_section_with_sep(Line::from("A"), Line::from(powerline::ARROW_RIGHT))
            .right_section_with_sep(Line::from("B"), Line::from(powerline::ARROW_LEFT))
            .center(Line::from("mid"));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// CJK wide characters in separator — width calculation must not panic.
    #[test]
    fn test_cjk_separator() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .left_section_with_sep(Line::from("left"), Line::from("《"));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    // --- Extreme builder chaining ---

    /// 100 left sections — must not panic.
    #[test]
    fn test_hundred_left_sections() {
        let mut builder = StatusLine::new();
        for i in 0..100 {
            builder = builder.left_section(Line::from(format!("{i}")));
        }
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        builder.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// 100 right sections — must not panic.
    #[test]
    fn test_hundred_right_sections() {
        let mut builder = StatusLine::new();
        for i in 0..100 {
            builder = builder.right_section(Line::from(format!("{i}")));
        }
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        builder.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// 100 left sections with separators — must not panic.
    #[test]
    fn test_hundred_left_sections_with_sep() {
        let mut builder = StatusLine::new();
        for i in 0..100 {
            builder = builder
                .left_section_with_sep(Line::from(format!("{i}")), Line::from("|"));
        }
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        builder.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    /// Very long center text — must not panic.
    #[test]
    fn test_very_long_center_text() {
        let long = "C".repeat(10_000);
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 1));
        let status = StatusLine::new().center(Line::from(long));
        status.render(Rect::new(0, 0, 80, 1), &mut buf);
    }

    // --- Style edge cases ---

    /// Empty Line sections (zero content width) — must not panic and should not render garbage.
    #[test]
    fn test_empty_line_sections() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .left_section(Line::from(""))
            .left_section_with_sep(Line::from(""), Line::from(""))
            .center(Line::from(""))
            .right_section(Line::from(""))
            .right_section_with_sep(Line::from(""), Line::from(""));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Default style with center margin renders correctly.
    #[test]
    fn test_default_style_center_margin_zero() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .center(Line::from("hello"))
            .style(StatusLineStyle::default());
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
        // Center text "hello" must appear somewhere in the buffer.
        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().to_string())
            .collect();
        assert!(content.contains('h'), "center text should be rendered");
    }

    /// Left section content exactly equal to area width — should fill buffer without panic.
    #[test]
    fn test_left_section_exact_width() {
        let content = "A".repeat(40);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().left_section(Line::from(content));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Right section content exactly equal to area width — should fill buffer without panic.
    #[test]
    fn test_right_section_exact_width() {
        let content = "B".repeat(40);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new().right_section(Line::from(content));
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// x_start arithmetic: left pass should not let x_start exceed area.right().
    /// If x_start overflows past x_end, subsequent Rect::new should not use an
    /// out-of-bounds coordinate that causes a buffer bounds panic.
    #[test]
    fn test_left_x_start_does_not_exceed_area_right() {
        // Two left sections that together overflow area.
        let content1 = "A".repeat(30);
        let content2 = "B".repeat(30);
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 1));
        let status = StatusLine::new()
            .left_section(Line::from(content1))
            .left_section(Line::from(content2));
        // x_start after section1 = 30, after section2 = 60, area.right() = 40.
        // x_start = 60 > area.right() = 40 when rendering section2.
        // Rect::new(60, 0, 30, 1) is out-of-buffer-bounds — potential panic.
        status.render(Rect::new(0, 0, 40, 1), &mut buf);
    }

    /// Verify that non-offset area (x > 0) does not cause coordinate arithmetic issues.
    #[test]
    fn test_non_zero_area_offset() {
        let mut buf = Buffer::empty(Rect::new(5, 2, 40, 1));
        let status = StatusLine::new()
            .left_section(Line::from("Left"))
            .right_section(Line::from("Right"))
            .center(Line::from("Mid"));
        status.render(Rect::new(5, 2, 40, 1), &mut buf);
    }
}
