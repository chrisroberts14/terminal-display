use crate::geometry::Rect;
use crate::style::Style;

/// A single terminal character position: one glyph and its visual style.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    /// The character displayed at this position.
    pub ch: char,
    /// The visual style (colours, bold, etc.) applied to `ch`.
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: ' ',
            style: Style::default(),
        }
    }
}

/// A 2D grid of [`Cell`]s covering a [`Rect`].
///
/// Widgets write into a `Buffer` during rendering. After each frame the terminal
/// compares the new buffer against the previous one with [`Buffer::diff`] and
/// only flushes the changed cells to stdout.
#[derive(Clone)]
pub struct Buffer {
    /// The region of the terminal this buffer covers.
    pub area: Rect,
    cells: Vec<Cell>,
    /// Set by animated widgets (e.g. [`Spinner`](crate::widget::Spinner)) during render
    /// to signal that the frame should be redrawn on a timer.
    pub(crate) animated: bool,
}

impl Buffer {
    /// Creates a buffer filled with blank [`Cell`]s covering `area`.
    pub fn empty(area: Rect) -> Self {
        let cell_count = area.width as usize * area.height as usize;
        Buffer {
            area,
            cells: vec![Cell::default(); cell_count],
            animated: false,
        }
    }

    /// Marks this buffer as requiring periodic redraws. Call from [`Widget::render`]
    /// implementations that display time-varying content.
    pub fn mark_animated(&mut self) {
        self.animated = true;
    }

    /// Converts absolute terminal coordinates to a flat index into `cells`.
    /// Returns `None` if `(x, y)` falls outside `self.area`.
    fn index(&self, x: u16, y: u16) -> Option<usize> {
        let rx = x.checked_sub(self.area.x)?;
        let ry = y.checked_sub(self.area.y)?;
        if rx >= self.area.width || ry >= self.area.height {
            return None;
        }
        Some(ry as usize * (self.area.width as usize) + rx as usize)
    }

    /// Returns the cell at absolute terminal position `(x, y)`.
    /// Returns `None` cell if the position is outside `self.area`.
    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        self.index(x, y).map(|index| self.cells[index].clone())
    }

    /// Writes `cell` to absolute terminal position `(x, y)`.
    /// Out-of-bounds writes are silently ignored.
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = cell;
        }
    }

    /// Writes each character of `s` left-to-right starting at `(x, y)`, all
    /// sharing the same `style`. Out-of-bounds characters are silently ignored.
    pub fn set_str(&mut self, x: u16, y: u16, s: &str, style: Style) {
        for (i, ch) in s.chars().enumerate() {
            self.set_cell(x + i as u16, y, Cell { ch, style });
        }
    }

    /// Returns `(x, y, cell)` for each cell that differs from `prev`.
    /// Returns `None` if the two buffers are not the same dimensions
    pub fn diff(&self, prev: &Buffer) -> Option<Vec<(u16, u16, Cell)>> {
        // Check dimensions and short circuit if they are not compatible
        if self.area.height != prev.area.height || self.area.width != prev.area.width {
            return None;
        }
        Some(
            self.cells
                .iter()
                .enumerate()
                .filter_map(|(i, cell)| {
                    let prev_cell = prev.cells.get(i)?;
                    if cell != prev_cell {
                        let x = self.area.x + (i as u16 % self.area.width);
                        let y = self.area.y + (i as u16 / self.area.width);
                        Some((x, y, cell.clone()))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;
    use crate::style::{Color, Style};

    fn make_buf(w: u16, h: u16) -> Buffer {
        Buffer::empty(Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        })
    }

    #[test]
    fn default_cell_is_space() {
        let c = Cell::default();
        assert_eq!(c.ch, ' ');
        assert_eq!(c.style, Style::default());
    }

    #[test]
    fn set_and_get_cell() {
        let mut buf = make_buf(10, 5);
        buf.set_cell(
            2,
            1,
            Cell {
                ch: 'X',
                style: Style::default(),
            },
        );
        assert_eq!(buf.get_cell(2, 1).unwrap().ch, 'X');
    }

    #[test]
    fn get_cell_out_of_bounds_returns_none() {
        let buf = make_buf(5, 5);
        assert_eq!(buf.get_cell(100, 100), None);
    }

    #[test]
    fn set_cell_out_of_bounds_is_ignored() {
        let mut buf = make_buf(5, 5);
        buf.set_cell(
            100,
            100,
            Cell {
                ch: 'X',
                style: Style::default(),
            },
        );
        // no panic
    }

    #[test]
    fn diff_returns_changed_cells() {
        let prev = make_buf(3, 1);
        let mut curr = make_buf(3, 1);
        curr.set_cell(
            1,
            0,
            Cell {
                ch: 'A',
                style: Style::default(),
            },
        );
        let diffs = curr.diff(&prev).unwrap();
        assert_eq!(diffs.len(), 1);
        assert_eq!(
            diffs[0],
            (
                1u16,
                0u16,
                Cell {
                    ch: 'A',
                    style: Style::default()
                }
            )
        );
    }

    #[test]
    fn diff_empty_when_identical() {
        let buf = make_buf(3, 1);
        assert!(buf.diff(&buf.clone()).unwrap().is_empty());
    }

    #[test]
    fn set_str_writes_chars_with_style() {
        let mut buf = make_buf(10, 1);
        let style = Style {
            fg: Some(Color::Red),
            ..Style::default()
        };
        buf.set_str(0, 0, "hi", style);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'h');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, 'i');
        assert_eq!(buf.get_cell(0, 0).unwrap().style.fg, Some(Color::Red));
    }
}
