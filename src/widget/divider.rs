use crate::{Buffer, Cell, Rect, Style, Widget};

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
