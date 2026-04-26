//! [`Table`] — renders rows of styled-span cells in a column grid.

use crate::buffer::{Buffer, Cell, char_width};
use crate::geometry::Rect;
use crate::layout::{self, Constraint};
use crate::style::{Color, Span, Style};
use crate::widget::Widget;

/// A single data row — each element is a cell rendered as a list of [`Span`]s.
pub struct Row {
    cells: Vec<Vec<Span>>,
}

impl Row {
    pub fn new(cells: Vec<Vec<Span>>) -> Self {
        Row { cells }
    }
}

/// Renders tabular data in a fixed column grid.
///
/// Column widths are controlled by [`Constraint`] values passed to [`Table::new`].
/// An optional header row is rendered in bold. An optional selected row index
/// highlights that row with a configurable background style.
pub struct Table {
    column_constraints: Vec<Constraint>,
    rows: Vec<Row>,
    headers: Option<Vec<String>>,
    selected: Option<usize>,
    highlight_style: Style,
}

impl Table {
    pub fn new(column_constraints: Vec<Constraint>, rows: Vec<Row>) -> Self {
        Table {
            column_constraints,
            rows,
            headers: None,
            selected: None,
            highlight_style: Style {
                bg: Some(Color::Blue),
                ..Style::default()
            },
        }
    }

    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }
}

impl Widget for Table {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let col_widths = layout::solve(&self.column_constraints, area.width);

        // Compute absolute x position for each column.
        let mut col_xs: Vec<u16> = vec![area.x; col_widths.len()];
        for i in 1..col_widths.len() {
            col_xs[i] = col_xs[i - 1].saturating_add(col_widths[i - 1]);
        }

        let mut row_y = area.y;

        // Render header row.
        if let Some(headers) = &self.headers
            && row_y < area.y + area.height
        {
            let header_style = Style {
                bold: true,
                ..Style::default()
            };
            for (i, header) in headers.iter().enumerate() {
                if i >= col_widths.len() {
                    break;
                }
                let mut clipped = String::new();
                let mut used = 0u16;
                for ch in header.chars() {
                    let w = char_width(ch);
                    if used + w > col_widths[i] {
                        break;
                    }
                    clipped.push(ch);
                    used += w;
                }
                buf.set_str(col_xs[i], row_y, &clipped, header_style);
            }
            row_y += 1;
        }

        // Render data rows.
        for (row_idx, row) in self.rows.iter().enumerate() {
            if row_y >= area.y + area.height {
                break;
            }

            let is_selected = self.selected == Some(row_idx);

            // Fill selected row background across the full width.
            if is_selected {
                buf.set_str(
                    area.x,
                    row_y,
                    &" ".repeat(area.width as usize),
                    self.highlight_style,
                );
            }

            // Render each cell's spans.
            for (col_idx, &col_w) in col_widths.iter().enumerate() {
                let x_start = col_xs[col_idx];
                let x_limit = x_start.saturating_add(col_w);

                if let Some(cell_spans) = row.cells.get(col_idx) {
                    let mut x_cursor = x_start;
                    'outer: for span in cell_spans {
                        let style = if is_selected {
                            span.style.patch(self.highlight_style)
                        } else {
                            span.style
                        };
                        for ch in span.content.chars() {
                            let w = char_width(ch);
                            if x_cursor.saturating_add(w) > x_limit {
                                break 'outer;
                            }
                            buf.set_cell(x_cursor, row_y, Cell { ch, style });
                            x_cursor = x_cursor.saturating_add(w);
                        }
                    }
                }
            }

            row_y += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::layout::Constraint;
    use crate::style::{Color, Span, Style};
    use crate::widget::Widget;

    fn area(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn headers_rendered_on_first_row() {
        // layout::solve([Fixed(8), Fill], 20) → [8, 12]
        // col_xs = [0, 8]
        // "Name" at (0, 0) bold; "Role" at (8, 0) bold.
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(vec![Constraint::Fixed(8), Constraint::Fill], vec![])
            .headers(vec!["Name".into(), "Role".into()])
            .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'N');
        assert!(buf.get_cell(0, 0).unwrap().style.bold);
        assert_eq!(buf.get_cell(8, 0).unwrap().ch, 'R');
        assert!(buf.get_cell(8, 0).unwrap().style.bold);
    }

    #[test]
    fn rows_start_at_y_plus_one_when_headers_present() {
        // Header at y=0, first data row at y=1.
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(
            vec![Constraint::Fixed(8), Constraint::Fill],
            vec![Row::new(vec![
                vec![Span::raw("Alice")],
                vec![Span::raw("Dev")],
            ])],
        )
        .headers(vec!["Name".into(), "Role".into()])
        .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'N'); // header 'N'ame
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, 'A'); // row 0 'A'lice
    }

    #[test]
    fn no_headers_rows_start_at_top() {
        // Without headers, first data row at y=0.
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(
            vec![Constraint::Fixed(8), Constraint::Fill],
            vec![Row::new(vec![
                vec![Span::raw("Alice")],
                vec![Span::raw("Dev")],
            ])],
        )
        .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'A');
    }

    #[test]
    fn selected_row_has_highlight_background() {
        // Row 0 selected with Red background; span has Green fg.
        // Selected row: bg=Red (highlight wins), fg=Green (span fg preserved).
        // Unselected row: bg=None, fg=None.
        let hl = Style {
            bg: Some(Color::Red),
            ..Style::default()
        };
        let green = Style {
            fg: Some(Color::Green),
            ..Style::default()
        };
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(
            vec![Constraint::Fixed(8), Constraint::Fill],
            vec![
                Row::new(vec![
                    vec![Span::styled("Alice", green)],
                    vec![Span::raw("Dev")],
                ]),
                Row::new(vec![vec![Span::raw("Bob")], vec![Span::raw("PM")]]),
            ],
        )
        .selected(Some(0))
        .highlight_style(hl)
        .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().style.bg, Some(Color::Red));
        assert_eq!(buf.get_cell(0, 0).unwrap().style.fg, Some(Color::Green));
        assert_eq!(buf.get_cell(0, 1).unwrap().style.bg, None);
    }

    #[test]
    fn column_constraints_respected() {
        // layout::solve([Fixed(5), Fill], 20) → [5, 15]
        // col_xs = [0, 5]
        // "ABCDEFGH" truncated to 5 chars → "ABCDE"; second column "X" starts at x=5.
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(
            vec![Constraint::Fixed(5), Constraint::Fill],
            vec![Row::new(vec![
                vec![Span::raw("ABCDEFGH")],
                vec![Span::raw("X")],
            ])],
        )
        .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(4, 0).unwrap().ch, 'E'); // last char of col 0
        assert_eq!(buf.get_cell(5, 0).unwrap().ch, 'X'); // col 1 start — NOT 'F'
    }

    #[test]
    fn span_style_applied_to_cell() {
        // Green styled span: cells 'O' and 'K' should carry fg=Green.
        let green = Style {
            fg: Some(Color::Green),
            ..Style::default()
        };
        let mut buf = Buffer::empty(area(20, 5));
        Table::new(
            vec![Constraint::Fixed(10), Constraint::Fill],
            vec![Row::new(vec![
                vec![Span::styled("OK", green)],
                vec![Span::raw("other")],
            ])],
        )
        .render(area(20, 5), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'O');
        assert_eq!(buf.get_cell(0, 0).unwrap().style.fg, Some(Color::Green));
        assert_eq!(buf.get_cell(1, 0).unwrap().ch, 'K');
        assert_eq!(buf.get_cell(1, 0).unwrap().style.fg, Some(Color::Green));
    }

    #[test]
    fn rows_beyond_area_height_clipped() {
        // area height = 3; 5 rows provided. Only rows 0-2 render.
        let mut buf = Buffer::empty(area(20, 3));
        Table::new(
            vec![Constraint::Fill],
            vec![
                Row::new(vec![vec![Span::raw("A")]]),
                Row::new(vec![vec![Span::raw("B")]]),
                Row::new(vec![vec![Span::raw("C")]]),
                Row::new(vec![vec![Span::raw("D")]]), // beyond area
                Row::new(vec![vec![Span::raw("E")]]), // beyond area
            ],
        )
        .render(area(20, 3), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'A');
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, 'B');
        assert_eq!(buf.get_cell(0, 2).unwrap().ch, 'C');
        assert!(buf.get_cell(0, 3).is_none()); // outside buffer bounds
    }

    #[test]
    fn zero_area_does_not_panic() {
        let zero = Rect {
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
        Table::new(
            vec![Constraint::Fill],
            vec![Row::new(vec![vec![Span::raw("X")]])],
        )
        .render(zero, &mut buf);
    }

    #[test]
    fn render_respects_non_zero_area_origin() {
        // area starts at (5, 3) — all content must be placed relative to that origin.
        // layout::solve([Fixed(8), Fill], 20) → col_xs = [5, 13]
        // Header 'N' at (5,3), 'R' at (13,3); data row 'A' at (5,4).
        let origin = Rect {
            x: 5,
            y: 3,
            width: 20,
            height: 5,
        };
        let mut buf = Buffer::empty(origin);
        Table::new(
            vec![Constraint::Fixed(8), Constraint::Fill],
            vec![Row::new(vec![
                vec![Span::raw("Alice")],
                vec![Span::raw("Dev")],
            ])],
        )
        .headers(vec!["Name".into(), "Role".into()])
        .render(origin, &mut buf);
        assert_eq!(buf.get_cell(5, 3).unwrap().ch, 'N');
        assert_eq!(buf.get_cell(13, 3).unwrap().ch, 'R');
        assert_eq!(buf.get_cell(5, 4).unwrap().ch, 'A');
    }

    #[test]
    fn wide_char_fits_exact_column_budget() {
        // Fixed(2) column — '中' (display width 2) exactly fills the budget, must render.
        let mut buf = Buffer::empty(area(4, 2));
        Table::new(
            vec![Constraint::Fixed(2), Constraint::Fill],
            vec![Row::new(vec![vec![Span::raw("中")], vec![Span::raw("x")]])],
        )
        .render(area(4, 2), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, '中');
    }

    #[test]
    fn wide_char_skipped_when_column_too_narrow() {
        // Fixed(1) column — '中' (display width 2) exceeds the budget, must not render.
        let mut buf = Buffer::empty(area(4, 2));
        Table::new(
            vec![Constraint::Fixed(1), Constraint::Fill],
            vec![Row::new(vec![vec![Span::raw("中")], vec![Span::raw("x")]])],
        )
        .render(area(4, 2), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, ' '); // cell untouched
    }
}
