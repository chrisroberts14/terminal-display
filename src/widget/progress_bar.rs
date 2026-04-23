use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::style::{Color, Span, Style};
use crate::widget::Widget;
use crate::widget::text::Text;

const FILL: &str = "█";
const EMPTY: &str = "░";

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
