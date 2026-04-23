//! Demonstrates the [`ProgressBar`] widget and the [`ProgressExt`] iterator adapter,
//! which automatically advances the bar as a `for` loop runs and completes it on exit.

use std::thread;
use std::time::Duration;
use terminal_display::{ProgressExt, Terminal};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let items: Vec<u32> = (1..=100).collect();

    for item in items.iter().with_progress(handle.clone()) {
        // Simulate work
        thread::sleep(Duration::from_millis(50));
        let _ = item;
    }

    // Bar is now at 100%. Give the user a moment to see it before exit.
    thread::sleep(Duration::from_millis(500));
}
