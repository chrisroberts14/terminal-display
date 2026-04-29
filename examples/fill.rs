//! Example of using the Fill widget
//! Displays a rainbow colour rectangle on the terminal

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Bordered, Color, Fill, HStack, Terminal, VStack, WidgetExt, style};

/// Construct a grid of RGB values
pub fn rgb_grid(width: usize, height: usize) -> Vec<Vec<(u8, u8, u8)>> {
    let mut grid = Vec::with_capacity(height);
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let r = (x as f32 / (width - 1).max(1) as f32) * 255.0;
            let g = (y as f32 / (height - 1).max(1) as f32) * 255.0;
            let b = 128.0;
            row.push((r as u8, g as u8, b as u8));
        }
        grid.push(row);
    }
    grid
}

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            Bordered::new(
                Block::new().title("Rainbow"),
                VStack::new(
                    rgb_grid(100, 100)
                        .into_iter()
                        .map(|row| {
                            HStack::new(
                                row.into_iter()
                                    .map(|(r, g, b)| {
                                        Fill::new(style!(bg = Color::Rgb(r, g, b))).fixed(1)
                                    })
                                    .collect(),
                            )
                            .fixed(1)
                        })
                        .collect(),
                ),
            ),
            area,
        )
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
