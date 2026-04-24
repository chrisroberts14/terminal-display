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

    fn natural_size(&self) -> Option<(u16, u16)> {
        self.child
            .natural_size()
            .map(|(w, h)| (w.saturating_add(2), h.saturating_add(2)))
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

    #[test]
    fn natural_size_adds_two_to_child_dimensions() {
        // Spinner is 1×1, so Bordered<Spinner> should report 3×3
        let widget = Bordered {
            block: Block::new(),
            child: crate::widget::spinner::Spinner::new(
                crate::widget::spinner::SpinnerStyle::Dots,
                crate::style::Style::default(),
            ),
        };
        assert_eq!(widget.natural_size(), Some((3, 3)));
    }

    #[test]
    fn natural_size_is_none_when_child_has_no_size() {
        struct NoSizeWidget;
        impl crate::widget::Widget for NoSizeWidget {
            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
        }

        let widget = Bordered {
            block: Block::new(),
            child: NoSizeWidget,
        };
        assert_eq!(widget.natural_size(), None);
    }

    #[test]
    fn natural_size_saturates_at_u16_max() {
        struct MaxSizeWidget;
        impl crate::widget::Widget for MaxSizeWidget {
            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
            fn natural_size(&self) -> Option<(u16, u16)> {
                Some((u16::MAX, u16::MAX))
            }
        }

        let widget = Bordered {
            block: Block::new(),
            child: MaxSizeWidget,
        };
        assert_eq!(widget.natural_size(), Some((u16::MAX, u16::MAX)));
    }
}
