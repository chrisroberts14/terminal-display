use std::thread;
use std::time::Duration;
use terminal_display::{span, style, Block, Color, Constraint, Terminal, Text, VStack, boxed};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let mut cpu = 0u32;
    let mut mem = 100u32;

    loop {
        handle.render(move |frame| {
            let area = frame.area();
            let block = Block::new().title("Stats");
            let inner = block.inner(area);
            frame.render(block, area);
            frame.render(
                VStack::new(vec![
                    (Constraint::Fixed(1), boxed(Text::raw(format!("CPU: {}%", cpu)))),
                    (Constraint::Fixed(1), boxed(Text::from(vec![
                        span!("MEM: "),
                        span!(format!("{}%", mem), style!(fg = Color::Red)),
                    ]))),
                ]),
                inner,
            );
        });

        cpu += 1;
        mem = mem.saturating_sub(1);

        thread::sleep(Duration::from_millis(500));
    }
}
