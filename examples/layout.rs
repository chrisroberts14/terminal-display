//! Demonstrates [`VStack`] and [`HStack`] layout with the three constraint types:
//! `Fixed` (exact size), `Ratio` (fractional share), and `Fill` (remaining space).

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Bordered, HStack, Terminal, Text, VStack, WidgetExt};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            // Outer VStack splits the screen into three horizontal bands
            VStack::new(vec![
                // Fixed: always 3 rows tall
                Bordered::new(
                    Block::new().title("Fixed(3)"),
                    Text::raw("This band is always 3 rows tall."),
                )
                .fixed(3),
                // Ratio: takes 1/3 of remaining space
                Bordered::new(
                    Block::new().title("Ratio(1,3)"),
                    HStack::new(vec![
                        Text::raw("left column").fill(),
                        Text::raw("right column").fill(),
                    ]),
                )
                .ratio(1, 3),
                // Fill: takes all remaining space
                Bordered::new(
                    Block::new().title("Fill"),
                    Text::raw("This band fills whatever is left."),
                )
                .fill(),
            ]),
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
