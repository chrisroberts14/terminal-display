//! Demonstrates [`Padding`] with all three constructor forms, each shown inside
//! a [`Bordered`] box. The blue fill shows the child area; the gap between the
//! border and the blue area is the padding.

use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Buffer, Color, HStack, Padding, Rect, Style, Terminal, Widget, WidgetExt,
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
            HStack::new(vec![
                Bordered::new(
                    Block::new().title("all(2)"),
                    Padding::all(2, ColorFill(Color::Blue)),
                )
                .fill(),
                Bordered::new(
                    Block::new().title("axes(4, 1)"),
                    Padding::axes(4, 1, ColorFill(Color::Green)),
                )
                .fill(),
                Bordered::new(
                    Block::new().title("new(0, 4, 2, 1)"),
                    Padding::new(0, 4, 2, 1, ColorFill(Color::Red)),
                )
                .fill(),
            ]),
            frame.area(),
        );
    });

    // Block until Ctrl-C.
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
