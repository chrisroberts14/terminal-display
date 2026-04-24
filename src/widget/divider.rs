use crate::{Buffer, Cell, Rect, Style, Widget};

/// A horizontal (`─`) or vertical (`│`) rule that fills its entire area.
///
/// Orientation is chosen automatically: horizontal when `width >= height`, vertical otherwise.
pub struct Divider {
    style: Style,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }

    pub fn styled(style: Style) -> Self {
        Self { style }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Divider {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        if area.width >= area.height {
            for x in area.x..area.x + area.width {
                buf.set_cell(
                    x,
                    area.y,
                    Cell {
                        ch: '─',
                        style: self.style,
                    },
                );
            }
        } else {
            for y in area.y..area.y + area.height {
                buf.set_cell(
                    area.x,
                    y,
                    Cell {
                        ch: '│',
                        style: self.style,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;

    fn rect(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn horizontal_fills_row_with_dash() {
        let mut buf = Buffer::empty(rect(5, 1));
        Divider::new().render(rect(5, 1), &mut buf);
        for x in 0..5 {
            assert_eq!(buf.get_cell(x, 0).unwrap().ch, '─');
        }
    }

    #[test]
    fn vertical_fills_column_with_pipe() {
        let mut buf = Buffer::empty(rect(1, 5));
        Divider::new().render(rect(1, 5), &mut buf);
        for y in 0..5 {
            assert_eq!(buf.get_cell(0, y).unwrap().ch, '│');
        }
    }

    #[test]
    fn square_area_draws_horizontal() {
        let mut buf = Buffer::empty(rect(4, 4));
        Divider::new().render(rect(4, 4), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '─');
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, ' '); // only first row drawn
    }

    #[test]
    fn styled_applies_to_cells() {
        let style = Style {
            bold: true,
            ..Style::default()
        };
        let mut buf = Buffer::empty(rect(3, 1));
        Divider::styled(style).render(rect(3, 1), &mut buf);
        assert!(buf.get_cell(0, 0).unwrap().style.bold);
        assert!(buf.get_cell(2, 0).unwrap().style.bold);
    }

    #[test]
    fn zero_width_does_not_panic() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 1,
        };
        let mut buf = Buffer::empty(Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        });
        Divider::new().render(area, &mut buf); // must not panic
    }
}
