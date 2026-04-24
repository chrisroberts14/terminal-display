use crate::{Buffer, Cell, Rect, Style, Widget};

/// A box drawn with Unicode line-drawing characters, with an optional title on the top border.
///
/// Typically used via [`Bordered`](crate::widget::Bordered) to frame another widget.
pub struct Block {
    title: Option<String>,
    border_style: Style,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Block {
            title: None,
            border_style: Style::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Returns the inner area inside the border
    pub fn inner(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        }
    }
}

impl Widget for Block {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        let style = self.border_style;
        let x0 = area.x;
        let y0 = area.y;
        let x1 = area.x + area.width - 1;
        let y1 = area.y + area.height - 1;

        buf.set_cell(x0, y0, Cell { ch: '┌', style });
        buf.set_cell(x1, y0, Cell { ch: '┐', style });
        buf.set_cell(x0, y1, Cell { ch: '└', style });
        buf.set_cell(x1, y1, Cell { ch: '┘', style });

        for x in (x0 + 1)..x1 {
            buf.set_cell(x, y0, Cell { ch: '─', style });
            buf.set_cell(x, y1, Cell { ch: '─', style });
        }
        for y in (y0 + 1)..y1 {
            buf.set_cell(x0, y, Cell { ch: '│', style });
            buf.set_cell(x1, y, Cell { ch: '│', style });
        }

        if let Some(title) = self.title.as_deref() {
            for (i, ch) in title.chars().enumerate() {
                let x = x0 + 1 + i as u16;
                if x >= x1 {
                    break;
                }
                buf.set_cell(x, y0, Cell { ch, style });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::widget::Widget;

    fn area(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn draws_corners() {
        let mut buf = Buffer::empty(area(5, 3));
        Block::new().render(area(5, 3), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '┌');
        assert_eq!(buf.get_cell(4, 0).unwrap().ch, '┐');
        assert_eq!(buf.get_cell(0, 2).unwrap().ch, '└');
        assert_eq!(buf.get_cell(4, 2).unwrap().ch, '┘');
    }

    #[test]
    fn draws_horizontal_borders() {
        let mut buf = Buffer::empty(area(5, 3));
        Block::new().render(area(5, 3), &mut buf);
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '─');
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, '─');
        assert_eq!(buf.get_cell(1, 2).unwrap().ch, '─');
    }

    #[test]
    fn draws_vertical_borders() {
        let mut buf = Buffer::empty(area(5, 4));
        Block::new().render(area(5, 4), &mut buf);
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, '│');
        assert_eq!(buf.get_cell(0, 2).unwrap().ch, '│');
        assert_eq!(buf.get_cell(4, 1).unwrap().ch, '│');
    }

    #[test]
    fn renders_title_after_top_left_corner() {
        let mut buf = Buffer::empty(area(10, 3));
        Block::new().title("CPU").render(area(10, 3), &mut buf);
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, 'C');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, 'P');
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, 'U');
    }

    #[test]
    fn inner_shrinks_by_one_on_all_sides() {
        let inner = Block::new().inner(area(10, 6));
        assert_eq!(
            inner,
            Rect {
                x: 1,
                y: 1,
                width: 8,
                height: 4
            }
        );
    }
}
