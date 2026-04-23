//! Demonstrates the [`Spinner`] widget with all three built-in styles side by side.

use std::thread;
use std::time::Duration;
use terminal_display::{Block, Bordered, HStack, Spinner, SpinnerStyle, Terminal, WidgetExt};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.animate(12);

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            HStack::new(vec![
                Bordered {
                    block: Block::new().title("Dots"),
                    child: Spinner::new(SpinnerStyle::Dots),
                }
                .fill(),
                Bordered {
                    block: Block::new().title("Line"),
                    child: Spinner::new(SpinnerStyle::Line),
                }
                .fill(),
                Bordered {
                    block: Block::new().title("Arc"),
                    child: Spinner::new(SpinnerStyle::Arc),
                }
                .fill(),
            ]),
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
