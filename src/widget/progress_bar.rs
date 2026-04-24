//! Horizontal progress bar widget.

use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::style::{Color, Span, Style};
use crate::widget::Widget;
use crate::widget::text::Text;

const FILL: &str = "█";
const EMPTY: &str = "░";

/// A horizontal progress bar rendered as `[████░░░░] 50% (5/10)`.
///
/// When `total` is `None` (e.g. for an iterator with an unknown length) the label
/// shows the item count instead of a percentage.
pub struct ProgressBar {
    current: usize,
    total: Option<usize>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new(0, None)
    }
}

impl ProgressBar {
    pub fn new(current: usize, total: Option<usize>) -> Self {
        ProgressBar { current, total }
    }
}

impl Widget for ProgressBar {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let label = match self.total {
            Some(total) if total > 0 => {
                let pct = (self.current * 100) / total;
                format!(" {}% ({}/{})", pct, self.current, total)
            }
            Some(_) => String::from(" 0% (0/0)"),
            None => format!(" {} items", self.current),
        };

        // Layout: [<bar>]<label>
        let label_width = label.len() as u16;
        let bar_width = area.width.saturating_sub(2 + label_width) as usize;

        let fill_count = match self.total {
            Some(total) if total > 0 && bar_width > 0 => {
                ((bar_width * self.current) / total).min(bar_width)
            }
            _ => 0,
        };
        let empty_count = bar_width.saturating_sub(fill_count);

        let green = Style {
            fg: Some(Color::Green),
            ..Style::default()
        };

        let mut spans = Vec::new();
        if bar_width > 0 {
            spans.push(Span::raw("["));
            spans.push(Span::styled(FILL.repeat(fill_count), green));
            spans.push(Span::raw(EMPTY.repeat(empty_count)));
            spans.push(Span::raw("]"));
        }
        spans.push(Span::raw(label));

        Text::from(spans).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;

    fn rect(w: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: 1,
        }
    }

    fn rendered(current: usize, total: Option<usize>, width: u16) -> Buffer {
        let area = rect(width);
        let mut buf = Buffer::empty(area);
        ProgressBar::new(current, total).render(area, &mut buf);
        buf
    }

    #[test]
    fn full_bar_first_cell_is_fill() {
        // 10/10 — bar should be entirely filled
        let buf = rendered(10, Some(10), 30);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '[');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '█');
    }

    #[test]
    fn empty_bar_first_cell_is_empty() {
        // 0/10 — bar should have no fill
        let buf = rendered(0, Some(10), 30);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '[');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '░');
    }

    #[test]
    fn half_bar_has_fill_then_empty() {
        // 5/10 at width 30: label=" 50% (5/10)"=11, bar_width=17, fill=8, empty=9
        let buf = rendered(5, Some(10), 30);
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '█'); // first fill cell
        assert_eq!(buf.get_cell(8, 0).unwrap().ch, '█'); // last fill cell (idx 8 = fill 8)
        assert_eq!(buf.get_cell(9, 0).unwrap().ch, '░'); // first empty cell
    }

    #[test]
    fn no_total_shows_item_count() {
        // Without a total the label starts with a space then the count
        let buf = rendered(7, None, 20);
        // Bar bracket at 0, after bar+] there is " 7 items" — just verify no panic and
        // that something was written
        assert_ne!(buf.get_cell(0, 0).unwrap().ch, ' ');
    }

    #[test]
    fn zero_area_does_not_panic() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut buf = Buffer::empty(Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        });
        ProgressBar::new(5, Some(10)).render(area, &mut buf);
    }

    #[test]
    fn closing_bracket_follows_bar() {
        // 0/10 at width 30: label=" 0% (0/10)"=10, bar_width=18, close bracket at pos 19
        let buf = rendered(0, Some(10), 30);
        assert_eq!(buf.get_cell(19, 0).unwrap().ch, ']');
    }
}
