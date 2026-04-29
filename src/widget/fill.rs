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
