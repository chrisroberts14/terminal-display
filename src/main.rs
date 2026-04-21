use std::thread;
use std::time::Duration;
use terminal_display::{Constraint, Terminal, Text, VStack, boxed};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let mut cpu = 0u32;
    let mut mem = 100u32;

    loop {
        handle.render(move |frame| {
            frame.render(
                VStack::new(vec![
                    (Constraint::Fixed(1), boxed(Text::raw(format!("CPU: {}%", cpu)))),
                    (Constraint::Fixed(1), boxed(Text::raw(format!("MEM: {}%", mem)))),
                ]),
                frame.area(),
            );
        });

        cpu += 1;
        mem = mem.saturating_sub(1);

        thread::sleep(Duration::from_millis(500));
    }
}
