//! Demonstrates the [`Text`] widget with a variety of span styles including bold, italic,
//! underline, foreground colour, and background colour.

use std::thread;
use std::time::Duration;
use terminal_display::{Color, Terminal, Text, span, style};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            Text::from(vec![
                span!("plain  "),
                span!("bold  ", style!(bold)),
                span!("italic  ", style!(italic)),
                span!("underline  ", style!(underline)),
                span!("red  ", style!(fg = Color::Red)),
                span!("on blue", style!(bg = Color::Blue)),
            ]),
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
