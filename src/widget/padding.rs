//! [`Padding`] — wraps any widget and insets its render area.

use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::widget::Widget;

/// Wraps any widget and insets its render area by a configurable amount on each side.
///
/// Use [`Padding::all`] for uniform padding, [`Padding::axes`] for separate
/// horizontal/vertical padding, or [`Padding::new`] for full per-side control.
pub struct Padding<W: Widget> {
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
    child: W,
}

impl<W: Widget> Padding<W> {
    /// Uniform padding on all four sides.
    pub fn all(amount: u16, child: W) -> Self {
        Self::new(amount, amount, amount, amount, child)
    }

    /// Separate horizontal (left + right) and vertical (top + bottom) padding.
    pub fn axes(horizontal: u16, vertical: u16, child: W) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal, child)
    }

    /// Full per-side control: top, right, bottom, left.
    pub fn new(top: u16, right: u16, bottom: u16, left: u16, child: W) -> Self {
        Padding {
            top,
            right,
            bottom,
            left,
            child,
        }
    }
}

impl<W: Widget + 'static> Widget for Padding<W> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let inner = Rect {
            x: area.x.saturating_add(self.left.min(area.width)),
            y: area.y.saturating_add(self.top.min(area.height)),
            width: area
                .width
                .saturating_sub(self.left.saturating_add(self.right)),
            height: area
                .height
                .saturating_sub(self.top.saturating_add(self.bottom)),
        };
        self.child.render(inner, buf);
    }

    fn natural_size(&self) -> Option<(u16, u16)> {
        self.child.natural_size().map(|(w, h)| {
            (
                w.saturating_add(self.left.saturating_add(self.right)),
                h.saturating_add(self.top.saturating_add(self.bottom)),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::style::Style;
    use crate::widget::Widget;
    use crate::widget::spinner::{Spinner, SpinnerStyle};
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
    fn all_insets_uniformly() {
        // all(2): child renders at (2, 2); cells at (0,0) and (1,2) stay blank.
        let mut buf = Buffer::empty(rect(10, 6));
        Padding::all(2, Text::raw("X")).render(rect(10, 6), &mut buf);
        assert_eq!(buf.get_cell(2, 2).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(1, 2).unwrap().ch, ' ');
    }

    #[test]
    fn axes_insets_independently() {
        // axes(horizontal=3, vertical=1): left=3, top=1 → child at (3, 1).
        let mut buf = Buffer::empty(rect(14, 5));
        Padding::axes(3, 1, Text::raw("X")).render(rect(14, 5), &mut buf);
        assert_eq!(buf.get_cell(3, 1).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(2, 1).unwrap().ch, ' ');
    }

    #[test]
    fn padding_larger_than_area_does_not_panic() {
        let area = rect(5, 5);
        let mut buf = Buffer::empty(area);
        Padding::all(100, Text::raw("X")).render(area, &mut buf);
        // Saturating sub collapses inner dimensions to zero — no panic.
    }

    #[test]
    fn natural_size_adds_padding_to_child_size() {
        // Spinner natural_size = Some((1, 1)).
        // all(2) adds left=2, right=2, top=2, bottom=2 → Some((5, 5)).
        let p = Padding::all(2, Spinner::new(SpinnerStyle::Dots, Style::default()));
        assert_eq!(p.natural_size(), Some((5, 5)));
    }

    #[test]
    fn natural_size_none_when_child_has_none() {
        struct NoSizeWidget;
        impl Widget for NoSizeWidget {
            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
        }
        let p = Padding::all(2, NoSizeWidget);
        assert_eq!(p.natural_size(), None);
    }

    #[test]
    fn natural_size_saturates_at_u16_max() {
        struct MaxSizeWidget;
        impl Widget for MaxSizeWidget {
            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
            fn natural_size(&self) -> Option<(u16, u16)> {
                Some((u16::MAX, u16::MAX))
            }
        }
        let p = Padding::all(10, MaxSizeWidget);
        assert_eq!(p.natural_size(), Some((u16::MAX, u16::MAX)));
    }

    #[test]
    fn render_respects_non_zero_area_origin() {
        // area starts at (5, 3); all(2) → child at (7, 5)
        let area = Rect {
            x: 5,
            y: 3,
            width: 10,
            height: 6,
        };
        let mut buf = Buffer::empty(area);
        Padding::all(2, Text::raw("X")).render(area, &mut buf);
        assert_eq!(buf.get_cell(7, 5).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(5, 3).unwrap().ch, ' ');
    }
}
