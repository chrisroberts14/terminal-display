use crate::terminal::Terminal;
use std::thread;
use std::time::Duration;

mod command;
mod terminal;

fn main() {
    let terminal = Terminal::new();
    let handle = terminal.run();

    let mut x = 0;
    let mut y = 100;

    loop {
        handle.set_line(0, format!("stat1: {}", x));
        handle.set_line(1, format!("stat2: {}", y));

        x += 1;
        y -= 1;

        thread::sleep(Duration::from_millis(500));
    }
}
