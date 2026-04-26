//! Demonstrates [`Popup`] rendering a centred dialog over a background layout.
//!
//! The background fills the screen with labelled rows. The popup appears centred
//! and grows to fit the [`Bordered`] dialog content. Press Ctrl-C to exit.

use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Buffer, Color, Padding, Popup, Rect, Style, Terminal, Text, VStack, Widget,
    WidgetExt,
};

/// Fills its entire render area with a solid background colour.
struct ColorFill(Color);

impl Widget for ColorFill {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let style = Style {
            bg: Some(self.0),
            ..Style::default()
        };
        for y in area.y..(area.y + area.height) {
            buf.set_str(area.x, y, &" ".repeat(area.width as usize), style);
        }
    }
}

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        frame.render(
            Popup::new(
                VStack::new(vec![
                    ColorFill(Color::BrightBlack).fill(),
                    Text::raw("Background content — row 1").fill(),
                    Text::raw("Background content — row 2").fill(),
                    Text::raw("Background content — row 3").fill(),
                    ColorFill(Color::BrightBlack).fill(),
                ]),
                Bordered::new(
                    Block::new().title("Popup"),
                    Padding::all(1, Text::raw("Hello from the popup!")),
                ),
            ),
            frame.area(),
        );
    });

    // Block until Ctrl-C.
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
