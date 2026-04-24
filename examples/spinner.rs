//! Demonstrates the [`Spinner`] widget with all three built-in styles side by side.

use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Color, HStack, Spinner, SpinnerStyle, Terminal, WidgetExt, style,
};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            HStack::new(vec![
                Bordered::new(
                    Block::new().title("Dots"),
                    Spinner::new(SpinnerStyle::Dots, style!(bold)),
                )
                .fill(),
                Bordered::new(
                    Block::new().title("Line"),
                    Spinner::new(SpinnerStyle::Line, style!(fg = Color::Green)),
                )
                .fill(),
                Bordered::new(
                    Block::new().title("Arc"),
                    Spinner::new(SpinnerStyle::Arc, style!(bg = Color::Red)),
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
