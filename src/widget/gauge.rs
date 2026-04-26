//! Ring gauge widget that fills clockwise from the top.

use std::f64::consts::PI;

use crate::Cell;
use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::style::Style;
use crate::widget::Widget;

const FILL: char = '█';
const EMPTY: char = '░';
const INNER_RATIO: f64 = 0.65;

/// A ring gauge rendered as a filled elliptical ring, filling clockwise from the
/// top according to `value`. A percentage label is drawn at the centre.
pub struct Gauge {
    value: f64,
    fill_style: Style,
}

impl Gauge {
    pub fn new(value: f64) -> Self {
        Gauge {
            value: value.clamp(0.0, 1.0),
            fill_style: Style::default(),
        }
    }

    pub fn fill_style(mut self, style: Style) -> Self {
        self.fill_style = style;
        self
    }
}

impl Widget for Gauge {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let cx = area.x as f64 + area.width as f64 / 2.0;
        let cy = area.y as f64 + area.height as f64 / 2.0;
        let half_w = area.width as f64 / 2.0;
        let half_h = area.height as f64 / 2.0;
        let filled_angle = self.value * 2.0 * PI;

        for row in area.y..(area.y + area.height) {
            for col in area.x..(area.x + area.width) {
                // +0.5 samples the centre of each cell, not its top-left corner.
                let dx = (col as f64 + 0.5 - cx) / half_w;
                let dy = (row as f64 + 0.5 - cy) / half_h * 2.0;
                let d = (dx * dx + dy * dy).sqrt();

                if !(INNER_RATIO..=1.0).contains(&d) {
                    continue;
                }

                // Clockwise angle from top: atan2(dx, -dy) gives 0 at top, π/2 at right.
                let mut angle = dx.atan2(-dy);
                if angle < 0.0 {
                    angle += 2.0 * PI;
                }

                if angle < filled_angle {
                    buf.set_cell(
                        col,
                        row,
                        Cell {
                            ch: FILL,
                            style: self.fill_style,
                        },
                    );
                } else {
                    buf.set_cell(
                        col,
                        row,
                        Cell {
                            ch: EMPTY,
                            style: Style::default(),
                        },
                    );
                }
            }
        }

        // Percentage label centred on the middle row, written over ring cells.
        let label = format!("{}%", (self.value * 100.0) as u32);
        let label_offset = area.width.saturating_sub(label.len() as u16) / 2;
        let label_row = area.y + area.height / 2;
        buf.set_str(area.x + label_offset, label_row, &label, Style::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::style::{Color, Style};
    use crate::widget::Widget;

    fn rect(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    fn rendered(value: f64, fill_style: Style, w: u16, h: u16) -> Buffer {
        let area = rect(w, h);
        let mut buf = Buffer::empty(area);
        Gauge::new(value)
            .fill_style(fill_style)
            .render(area, &mut buf);
        buf
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
        Gauge::new(0.5).render(area, &mut buf);
    }

    #[test]
    fn zero_value_no_fill() {
        let buf = rendered(0.0, Style::default(), 30, 15);
        for y in 0..15 {
            for x in 0..30 {
                if let Some(cell) = buf.get_cell(x, y) {
                    assert_ne!(cell.ch, FILL, "found fill char at ({x},{y}) with value=0.0");
                }
            }
        }
    }

    #[test]
    fn full_value_no_empty_ring_cells() {
        // At value=1.0 every ring cell should be filled — no ░ should appear.
        let buf = rendered(1.0, Style::default(), 30, 15);
        let has_fill = (0..30u16)
            .flat_map(|x| (0..15u16).map(move |y| (x, y)))
            .filter_map(|(x, y)| buf.get_cell(x, y))
            .any(|c| c.ch == FILL);
        assert!(has_fill, "no fill chars found at value=1.0");
        for y in 0..15 {
            for x in 0..30 {
                if let Some(cell) = buf.get_cell(x, y) {
                    assert_ne!(
                        cell.ch, EMPTY,
                        "found empty ring char at ({x},{y}) with value=1.0"
                    );
                }
            }
        }
    }

    #[test]
    fn label_shows_percentage() {
        let buf = rendered(0.5, Style::default(), 30, 15);
        let center_y = 15u16 / 2;
        let row: String = (0..30u16)
            .filter_map(|x| buf.get_cell(x, center_y))
            .map(|c| c.ch)
            .collect();
        assert!(row.contains('%'), "no % found in centre row: {row:?}");
        assert!(row.contains('5'), "no 5 found in centre row: {row:?}");
        assert!(row.contains('0'), "no 0 found in centre row: {row:?}");
    }

    #[test]
    fn fill_style_applied_to_filled_cells() {
        let green = Style {
            fg: Some(Color::Green),
            ..Style::default()
        };
        let buf = rendered(1.0, green, 30, 15);
        let has_green = (0..30u16)
            .flat_map(|x| (0..15u16).map(move |y| (x, y)))
            .filter_map(|(x, y)| buf.get_cell(x, y))
            .filter(|c| c.ch == FILL)
            .any(|c| c.style.fg == Some(Color::Green));
        assert!(has_green, "fill_style not applied to any filled cell");
    }
}
