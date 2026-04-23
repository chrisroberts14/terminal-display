//! Demonstrates the [`Block`] widget: a bordered box with an optional title. Shows how to
//! manually obtain the inner area and render a child widget inside it.

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Terminal, Text};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        let block = Block::new().title("My Block");
        let inner = block.inner(area);
        frame.render(block, area);
        frame.render(Text::raw("Content inside the block"), inner);
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
