//! A filled in box widget

use crate::{Buffer, Cell, Rect, Style, Widget};

/// A box that can be drawn with an optional fill colour
pub struct Fill {
    style: Style,
}

impl Fill {
    pub fn new(style: Style) -> Self {
        Fill { style }
    }
}

impl Widget for Fill {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_cell(
                    x,
                    y,
                    Cell {
                        ch: ' ',
                        style: self.style,
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, style};

    fn area(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn test_colour_is_rendered() {
        let mut buffer = Buffer::empty(area(1, 1));
        Fill::new(style!(bg = Color::Black)).render(area(1, 1), &mut buffer);
        assert_eq!(buffer.get_cell(0, 0).unwrap().ch, ' ');
        assert_eq!(
            buffer.get_cell(0, 0).unwrap().style,
            style!(bg = Color::Black)
        );
    }
}
