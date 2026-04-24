//! [`Bordered`] — wraps any widget in a [`Block`] border.

use crate::widget::block::Block;
use crate::{Buffer, Rect, Widget};

/// Wraps any widget in a [`Block`] border, rendering the border first then the
/// child clipped to the inner area.
pub struct Bordered<W: Widget> {
    pub block: Block,
    pub child: W,
}

impl<W: Widget + 'static> Widget for Bordered<W> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let inner = self.block.inner(area);
        self.block.render(area, buf);
        self.child.render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::widget::block::Block;
    use crate::widget::text::Text;

    fn rect(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn renders_border_corners() {
        let mut buf = Buffer::empty(rect(10, 5));
        Bordered {
            block: Block::new(),
            child: Text::raw(""),
        }
        .render(rect(10, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '┌');
        assert_eq!(buf.get_cell(9, 0).unwrap().ch, '┐');
        assert_eq!(buf.get_cell(0, 4).unwrap().ch, '└');
        assert_eq!(buf.get_cell(9, 4).unwrap().ch, '┘');
    }

    #[test]
    fn child_renders_inside_border() {
        let mut buf = Buffer::empty(rect(10, 5));
        Bordered {
            block: Block::new(),
            child: Text::raw("hi"),
        }
        .render(rect(10, 5), &mut buf);
        // Inner area starts at (1,1)
        assert_eq!(buf.get_cell(1, 1).unwrap().ch, 'h');
        assert_eq!(buf.get_cell(2, 1).unwrap().ch, 'i');
    }

    #[test]
    fn child_does_not_overwrite_border() {
        let long_text = "x".repeat(20);
        let mut buf = Buffer::empty(rect(10, 5));
        Bordered {
            block: Block::new(),
            child: Text::raw(long_text),
        }
        .render(rect(10, 5), &mut buf);
        // Right border must still be '│', not overwritten by the long child text
        assert_eq!(buf.get_cell(9, 1).unwrap().ch, '│');
    }

    #[test]
    fn title_appears_on_top_border() {
        let mut buf = Buffer::empty(rect(10, 5));
        Bordered {
            block: Block::new().title("CPU"),
            child: Text::raw(""),
        }
        .render(rect(10, 5), &mut buf);
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, 'C');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, 'P');
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, 'U');
    }
}
