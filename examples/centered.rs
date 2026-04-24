//! Demonstrates the [`Centered`] wrapper, which places any child widget in the middle
//! of its allocated area using the child's [`natural_size`](terminal_display::Widget::natural_size).

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Bordered, Centered, Spinner, SpinnerStyle, Terminal, style};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            Bordered::new(
                Block::new().title("Loading"),
                Centered::new(Spinner::new(SpinnerStyle::Dots, style!(bold))),
            ),
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
