//! Single-line styled text widget.

use crate::buffer::char_width;
use crate::widget::Widget;
use crate::{Buffer, Cell, Rect, Span};

/// A single line of styled text, composed of one or more [`Span`]s.
///
/// Content that overflows the area width is clipped. Newlines are not supported —
/// use multiple widgets in a [`VStack`](crate::widget::VStack) for multi-line layouts.
pub struct Text {
    spans: Vec<Span>,
}

impl Text {
    pub fn from(spans: Vec<Span>) -> Self {
        Text { spans }
    }

    pub fn raw(content: impl Into<String>) -> Self {
        Text::from(vec![Span::raw(content)])
    }
}

impl Widget for Text {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;
        'outer: for span in &self.spans {
            for ch in span.content.chars() {
                let w = char_width(ch);
                if x.saturating_add(w) > area.x + area.width {
                    break 'outer;
                }
                buf.set_cell(
                    x,
                    area.y,
                    Cell {
                        ch,
                        style: span.style,
                    },
                );
                x = x.saturating_add(w);
            }
        }
    }

    fn natural_size(&self) -> Option<(u16, u16)> {
        let total = self
            .spans
            .iter()
            .flat_map(|s| s.content.chars())
            .fold(0u16, |acc, ch| acc.saturating_add(char_width(ch)));
        Some((total, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::style::{Color, Span, Style};
    use crate::widget::Widget;

    fn area(w: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: 1,
        }
    }

    #[test]
    fn renders_plain_span() {
        let mut buf = Buffer::empty(area(10));
        Text::from(vec![Span::raw("hello")]).render(area(10), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'h');
        assert_eq!(buf.get_cell(4, 0).unwrap().ch, 'o');
        assert_eq!(buf.get_cell(5, 0).unwrap().ch, ' '); // unfilled cell stays space
    }

    #[test]
    fn clips_to_area_width() {
        let mut buf = Buffer::empty(area(3));
        Text::from(vec![Span::raw("hello")]).render(area(3), &mut buf);
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, 'l');
        // position 3 doesn't exist — no panic
    }

    #[test]
    fn multiple_spans_with_different_styles() {
        let red = Style {
            fg: Some(Color::Red),
            ..Style::default()
        };
        let mut buf = Buffer::empty(area(10));
        Text::from(vec![Span::raw("ab"), Span::styled("cd", red)]).render(area(10), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'a');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, 'c');
        assert_eq!(buf.get_cell(2, 0).unwrap().style.fg, Some(Color::Red));
        assert_eq!(buf.get_cell(0, 0).unwrap().style.fg, None);
    }

    #[test]
    fn natural_size_sums_chars_across_spans() {
        let t = Text::from(vec![Span::raw("hello"), Span::raw("world")]);
        assert_eq!(t.natural_size(), Some((10, 1)));
    }

    #[test]
    fn natural_size_of_empty_spans_is_zero_width() {
        let t = Text::from(vec![]);
        assert_eq!(t.natural_size(), Some((0, 1)));
    }

    #[test]
    fn natural_size_counts_display_columns_not_bytes() {
        // "café" is 4 Unicode scalar values (each width 1) but 5 UTF-8 bytes → 4 columns
        let t = Text::raw("café");
        assert_eq!(t.natural_size(), Some((4, 1)));
    }

    #[test]
    fn wide_char_at_area_right_edge_is_clipped() {
        // 2-wide area: 'a' fits at x=0, '中' (width 2) would need x=1..=2 but area ends at x=1 → skipped
        let area = Rect {
            x: 0,
            y: 0,
            width: 2,
            height: 1,
        };
        let mut buf = Buffer::empty(area);
        Text::raw("a中").render(area, &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'a');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, ' '); // wide char was not written
    }

    #[test]
    fn natural_size_counts_display_columns() {
        // "中文" = 2 CJK chars, each display width 2 → total 4 columns
        let t = Text::raw("中文");
        assert_eq!(t.natural_size(), Some((4, 1)));
    }

    #[test]
    fn render_wide_text_correct_columns() {
        // "a中b": 'a' at x=0, '中' at x=1 (width 2), 'b' at x=3 (key assertion — not x=2)
        let area = Rect {
            x: 0,
            y: 0,
            width: 6,
            height: 1,
        };
        let mut buf = Buffer::empty(area);
        Text::raw("a中b").render(area, &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'a');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '中');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, '\0'); // Buffer::set_cell continuation sentinel
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, 'b');
    }
}
