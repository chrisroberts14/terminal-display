//! Single-line styled text widget.

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
                if x >= area.x + area.width {
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
                x += 1;
            }
        }
    }

    fn natural_size(&self) -> Option<(u16, u16)> {
        let total: usize = self.spans.iter().map(|s| s.content.chars().count()).sum();
        Some((u16::try_from(total).unwrap_or(u16::MAX), 1))
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
    fn natural_size_counts_unicode_chars_not_bytes() {
        // "café" is 4 Unicode scalar values but 5 UTF-8 bytes
        let t = Text::raw("café");
        assert_eq!(t.natural_size(), Some((4, 1)));
    }
}
