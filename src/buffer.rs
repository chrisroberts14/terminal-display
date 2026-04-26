//! The [`Buffer`] type — a 2D grid of [`Cell`]s that widgets render into.

use crate::geometry::Rect;
use crate::style::Style;

/// Returns the display column-width of `ch` for buffer advancement purposes (always 1 or 2).
/// `None`-width characters (most control chars) default to 1 via `unwrap_or(1)`.
/// Zero-width characters (combining marks, `'\0'`) are clamped to 1 via `.max(1)`
/// so that buffer position arithmetic never stalls.
pub(crate) fn char_width(ch: char) -> u16 {
    unicode_width::UnicodeWidthChar::width(ch)
        .unwrap_or(1)
        .max(1) as u16
}

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
/// compares the new buffer against the previous one and only flushes the changed
/// cells to stdout.
#[derive(Clone)]
pub struct Buffer {
    /// The region of the terminal this buffer covers.
    pub(crate) area: Rect,
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

    /// Marks this buffer as requiring periodic redraws. Call from
    /// [`Widget::render`](crate::widget::Widget::render) implementations that display
    /// time-varying content.
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
    /// If `cell.ch` has display width 2, also writes a `'\0'` continuation sentinel at `(x+1, y)`.
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(index) = self.index(x, y) {
            let cell_ch = cell.ch;
            let cell_style = cell.style;
            self.cells[index] = cell;
            if char_width(cell_ch) == 2
                && let Some(cont_index) = self.index(x.saturating_add(1), y)
            {
                self.cells[cont_index] = Cell {
                    ch: '\0',
                    style: cell_style,
                };
            }
        }
    }

    /// Writes each character of `s` left-to-right starting at `(x, y)`, all
    /// sharing the same `style`. Advances `x` by each character's display width.
    /// Out-of-bounds characters are silently ignored.
    pub fn set_str(&mut self, mut x: u16, y: u16, s: &str, style: Style) {
        for ch in s.chars() {
            self.set_cell(x, y, Cell { ch, style });
            x = x.saturating_add(char_width(ch));
        }
    }

    /// Returns `(x, y, cell)` for each cell that differs from `prev`.
    /// Returns `None` if the two buffers are not the same dimensions
    pub(crate) fn diff(&self, prev: &Buffer) -> Option<Vec<(u16, u16, Cell)>> {
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

    #[test]
    fn set_cell_wide_char_writes_continuation() {
        // '中' has display width 2; writing it at (0,0) must auto-fill (1,0) with '\0'.
        let mut buf = make_buf(4, 1);
        buf.set_cell(
            0,
            0,
            Cell {
                ch: '中',
                style: Style::default(),
            },
        );
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '中');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '\0');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, ' '); // untouched
    }

    #[test]
    fn set_str_wide_char_advances_correctly() {
        // "中a": '中' at column 0 (width 2), 'a' at column 2 (not column 1).
        let mut buf = make_buf(5, 1);
        buf.set_str(0, 0, "中a", Style::default());
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '中');
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, '\0');
        assert_eq!(buf.get_cell(2, 0).unwrap().ch, 'a');
    }

    #[test]
    fn diff_shows_continuation_cell_for_wide_char() {
        // buffer::diff() includes the '\0' cell — render_diff must skip it.
        let area = Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        };
        let prev = Buffer::empty(area);
        let mut curr = Buffer::empty(area);
        let red = Style {
            fg: Some(Color::Red),
            ..Style::default()
        };
        curr.set_cell(
            0,
            0,
            Cell {
                ch: '中',
                style: red,
            },
        );
        let diffs = curr.diff(&prev).unwrap();
        let cont = diffs
            .iter()
            .find(|(x, _, _)| *x == 1)
            .expect("no continuation cell in diff");
        assert_eq!(cont.2.ch, '\0');
        assert_eq!(cont.2.style.fg, Some(Color::Red)); // style is copied from the wide cell
    }

    #[test]
    fn set_cell_wide_char_at_right_edge_does_not_panic_or_write_outside() {
        // '中' at x=3 in a width=4 buffer: continuation at x=4 is out-of-bounds, silently dropped.
        let mut buf = make_buf(4, 1);
        buf.set_cell(
            3,
            0,
            Cell {
                ch: '中',
                style: Style::default(),
            },
        );
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, '中');
        assert!(buf.get_cell(4, 0).is_none());
    }
}
