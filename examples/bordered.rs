//! Demonstrates the [`Bordered`] wrapper, which composes a [`Block`] border with any child
//! widget and handles the inner-area calculation automatically.

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Bordered, Terminal, Text};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            Bordered::new(
                Block::new().title("Bordered widget"),
                Text::raw("The Bordered wrapper handles the border and renders the child in the inner area automatically."),
            ),
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
