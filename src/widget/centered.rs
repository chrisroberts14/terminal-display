//! [`Centered`] — renders a child widget in the middle of the available area.

use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::widget::Widget;

/// Wraps any widget and renders it centred within the available area.
///
/// Centering uses [`Widget::natural_size`]. If the child returns `None`,
/// it is rendered into the full area unchanged.
pub struct Centered<W: Widget> {
    child: W,
}

impl<W: Widget> Centered<W> {
    pub fn new(child: W) -> Self {
        Centered { child }
    }
}

impl<W: Widget> Widget for Centered<W> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let render_area = match self.child.natural_size() {
            Some((child_w, child_h)) => {
                let x = area.x + area.width.saturating_sub(child_w) / 2;
                let y = area.y + area.height.saturating_sub(child_h) / 2;
                let width = child_w.min(area.width);
                let height = child_h.min(area.height);
                Rect {
                    x,
                    y,
                    width,
                    height,
                }
            }
            None => area,
        };
        self.child.render(render_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::{Buffer, Cell};
    use crate::geometry::Rect;
    use crate::style::Style;
    use crate::widget::Widget;
    use crate::widget::block::Block;
    use crate::widget::bordered::Bordered;
    use crate::widget::spinner::{Spinner, SpinnerStyle};

    fn rect(x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect {
            x,
            y,
            width: w,
            height: h,
        }
    }

    // A widget with no natural_size that writes 'X' at the top-left of whatever
    // area it receives — used to verify Centered passes the full area through.
    struct FullAreaMarker;
    impl Widget for FullAreaMarker {
        fn render(&self, area: Rect, buf: &mut Buffer) {
            buf.set_cell(
                area.x,
                area.y,
                Cell {
                    ch: 'X',
                    style: Style::default(),
                },
            );
        }
    }

    #[test]
    fn centered_spinner_renders_at_center_of_5x5() {
        // Spinner natural_size = Some((1, 1))
        // x = 0 + (5 - 1) / 2 = 2,  y = 0 + (5 - 1) / 2 = 2
        let area = rect(0, 0, 5, 5);
        let mut buf = Buffer::empty(area);
        Centered::new(Spinner::new(SpinnerStyle::Line, Style::default())).render(area, &mut buf);
        let ch = buf.get_cell(2, 2).unwrap().ch;
        assert!(ch != ' ', "expected a spinner char at (2,2), got space");
        assert_eq!(buf.get_cell(1, 2).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(3, 2).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(2, 1).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(2, 3).unwrap().ch, ' ');
    }

    #[test]
    fn centered_spinner_renders_at_center_of_7x3() {
        // x = 0 + (7 - 1) / 2 = 3,  y = 0 + (3 - 1) / 2 = 1
        let area = rect(0, 0, 7, 3);
        let mut buf = Buffer::empty(area);
        Centered::new(Spinner::new(SpinnerStyle::Line, Style::default())).render(area, &mut buf);
        let ch = buf.get_cell(3, 1).unwrap().ch;
        assert!(ch != ' ', "expected a spinner char at (3,1), got space");
    }

    #[test]
    fn none_child_renders_into_full_area() {
        // FullAreaMarker returns None from natural_size (trait default).
        // Centered must pass the full area through — 'X' ends up at (0, 0).
        let area = rect(0, 0, 5, 5);
        let mut buf = Buffer::empty(area);
        Centered::new(FullAreaMarker).render(area, &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'X');
    }

    #[test]
    fn centered_bordered_spinner_places_3x3_box_in_center_of_7x7() {
        // Spinner natural_size = Some((1, 1))
        // Bordered<Spinner> natural_size = Some((3, 3))
        // x = 0 + (7 - 3) / 2 = 2,  y = 0 + (7 - 3) / 2 = 2
        // Sub-rect: Rect { x: 2, y: 2, width: 3, height: 3 }
        // Block corners: '┌' at (2,2), '┐' at (4,2), '└' at (2,4), '┘' at (4,4)
        // Inner area: Rect { x: 3, y: 3, width: 1, height: 1 }
        // Spinner char at (3, 3)
        let area = rect(0, 0, 7, 7);
        let mut buf = Buffer::empty(area);
        Centered::new(Bordered::new(
            Block::new(),
            Spinner::new(SpinnerStyle::Line, Style::default()),
        ))
        .render(area, &mut buf);
        assert_eq!(buf.get_cell(2, 2).unwrap().ch, '┌');
        assert_eq!(buf.get_cell(4, 2).unwrap().ch, '┐');
        assert_eq!(buf.get_cell(2, 4).unwrap().ch, '└');
        assert_eq!(buf.get_cell(4, 4).unwrap().ch, '┘');
        let spinner_ch = buf.get_cell(3, 3).unwrap().ch;
        assert!(
            spinner_ch != ' ',
            "expected a spinner char at (3,3), got space"
        );
        // Cells outside the 3×3 box must be untouched
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' ');
        assert_eq!(buf.get_cell(6, 6).unwrap().ch, ' ');
    }

    #[test]
    fn oversized_child_clamps_to_area_origin() {
        // Text "hello world" is 11 chars wide but area is only 5×1.
        // saturating_sub clamps offset to 0, width clamps to area.width.
        // 'h' should appear at (0, 0).
        use crate::widget::text::Text;
        let area = rect(0, 0, 5, 1);
        let mut buf = Buffer::empty(area);
        Centered::new(Text::raw("hello world")).render(area, &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'h');
        assert_eq!(buf.get_cell(4, 0).unwrap().ch, 'o');
    }
}
