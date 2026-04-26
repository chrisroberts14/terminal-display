//! [`Popup`] — renders a background widget and an overlay widget centred on top.

use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::widget::Widget;

/// Renders a background widget into the full area, then renders an overlay widget
/// centred on top.
///
/// The popup rect is at least 20 % × 20 % of the available area and grows to fit
/// the overlay's [`Widget::natural_size`] when that is larger. If the overlay
/// returns `None` from `natural_size`, the 20 % × 20 % default is used.
pub struct Popup<B: Widget, O: Widget> {
    background: B,
    overlay: O,
}

impl<B: Widget, O: Widget> Popup<B, O> {
    pub fn new(background: B, overlay: O) -> Self {
        Popup {
            background,
            overlay,
        }
    }
}

impl<B: Widget, O: Widget> Widget for Popup<B, O> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.background.render(area, buf);

        if area.width == 0 || area.height == 0 {
            return;
        }

        let default_w = ((area.width as u32 * 20 / 100) as u16).max(1);
        let default_h = ((area.height as u32 * 20 / 100) as u16).max(1);

        let (popup_w, popup_h) = match self.overlay.natural_size() {
            Some((w, h)) => (w.max(default_w), h.max(default_h)),
            None => (default_w, default_h),
        };

        let popup_w = popup_w.min(area.width);
        let popup_h = popup_h.min(area.height);

        let x = area
            .x
            .saturating_add(area.width.saturating_sub(popup_w) / 2);
        let y = area
            .y
            .saturating_add(area.height.saturating_sub(popup_h) / 2);

        self.overlay.render(
            Rect {
                x,
                y,
                width: popup_w,
                height: popup_h,
            },
            buf,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::{Buffer, Cell};
    use crate::geometry::Rect;
    use crate::style::Style;
    use crate::widget::Widget;
    use crate::widget::text::Text;

    fn rect(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    struct NoOp;
    impl Widget for NoOp {
        fn render(&self, _area: Rect, _buf: &mut Buffer) {}
    }

    struct MarkerAt(char);
    impl Widget for MarkerAt {
        fn render(&self, area: Rect, buf: &mut Buffer) {
            buf.set_cell(
                area.x,
                area.y,
                Cell {
                    ch: self.0,
                    style: Style::default(),
                },
            );
        }
    }

    struct Fill(char);
    impl Widget for Fill {
        fn render(&self, area: Rect, buf: &mut Buffer) {
            for y in area.y..(area.y + area.height) {
                for x in area.x..(area.x + area.width) {
                    buf.set_cell(
                        x,
                        y,
                        Cell {
                            ch: self.0,
                            style: Style::default(),
                        },
                    );
                }
            }
        }
    }

    #[test]
    fn overlay_natural_size_used_when_larger_than_default() {
        // Area 30×10: default_w = 30*20/100 = 6, default_h = 10*20/100 = 2.
        // Text "0123456789ABCDE" natural_size = Some((15, 1)).
        // popup_w = max(6, 15) = 15, popup_h = max(2, 1) = 2.
        // x = 0 + (30-15)/2 = 7, y = 0 + (10-2)/2 = 4.
        // '0' rendered at (7, 4).
        let area = rect(30, 10);
        let mut buf = Buffer::empty(area);
        Popup::new(NoOp, Text::raw("0123456789ABCDE")).render(area, &mut buf);
        assert_eq!(buf.get_cell(7, 4).unwrap().ch, '0');
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
    }

    #[test]
    fn overlay_smaller_than_default_uses_default_size() {
        // Area 30×10: default_w = 6, default_h = 2.
        // Text "X" natural_size = Some((1, 1)).
        // popup_w = max(6, 1) = 6, popup_h = max(2, 1) = 2.
        // x = 0 + (30-6)/2 = 12, y = 0 + (10-2)/2 = 4.
        // 'X' rendered at (12, 4).
        let area = rect(30, 10);
        let mut buf = Buffer::empty(area);
        Popup::new(NoOp, Text::raw("X")).render(area, &mut buf);
        assert_eq!(buf.get_cell(12, 4).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
    }

    #[test]
    fn no_natural_size_uses_default_size() {
        // Area 30×10: default_w = 6, default_h = 2.
        // MarkerAt has no natural_size → popup uses (6, 2).
        // x = 12, y = 4. 'X' at (12, 4).
        let area = rect(30, 10);
        let mut buf = Buffer::empty(area);
        Popup::new(NoOp, MarkerAt('X')).render(area, &mut buf);
        assert_eq!(buf.get_cell(12, 4).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
    }

    #[test]
    fn popup_clamped_to_area_does_not_panic() {
        // Area 5×5, overlay natural_size = Some((100, 100)).
        // popup dims clamped to (5, 5). No panic.
        struct HugeOverlay;
        impl Widget for HugeOverlay {
            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
            fn natural_size(&self) -> Option<(u16, u16)> {
                Some((100, 100))
            }
        }
        let area = rect(5, 5);
        let mut buf = Buffer::empty(area);
        Popup::new(NoOp, HugeOverlay).render(area, &mut buf);
    }

    #[test]
    fn background_rendered_behind_overlay() {
        // Fill background with 'B'. Overlay writes 'O' at its top-left.
        // Area 20×10: default_w = 4, default_h = 2.
        // Text "O" natural_size = Some((1, 1)).
        // popup_w = max(4, 1) = 4, popup_h = max(2, 1) = 2.
        // x = 0 + (20-4)/2 = 8, y = 0 + (10-2)/2 = 4.
        // 'O' at (8, 4); 'B' survives at (0, 0) where overlay does not write.
        let area = rect(20, 10);
        let mut buf = Buffer::empty(area);
        Popup::new(Fill('B'), Text::raw("O")).render(area, &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'B');
        assert_eq!(buf.get_cell(8, 4).unwrap().ch, 'O');
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
        Popup::new(NoOp, Text::raw("X")).render(area, &mut buf);
    }

    #[test]
    fn render_respects_non_zero_area_origin() {
        // area starts at (10, 5), size 30×10.
        // default_w = 6, default_h = 2.
        // Text "X" natural_size = Some((1, 1)) → popup_w = 6, popup_h = 2.
        // x = 10 + (30-6)/2 = 10 + 12 = 22, y = 5 + (10-2)/2 = 5 + 4 = 9.
        let area = Rect {
            x: 10,
            y: 5,
            width: 30,
            height: 10,
        };
        let mut buf = Buffer::empty(area);
        Popup::new(NoOp, Text::raw("X")).render(area, &mut buf);
        assert_eq!(buf.get_cell(22, 9).unwrap().ch, 'X');
        assert_eq!(buf.get_cell(10, 5).unwrap().ch, ' ');
    }
}
