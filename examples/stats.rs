//! A live-updating stats dashboard combining [`Block`], [`Bordered`], [`HStack`], [`VStack`],
//! and styled [`Text`] spans. Simulates CPU and memory counters updating every 500 ms.

use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Color, HStack, Terminal, Text, VStack, WidgetExt, span, style,
};

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
                HStack::new(vec![
                    Bordered {
                        block: Block::new().title("Core 0"),
                        child: VStack::new(vec![
                            Text::raw(format!("CPU: {}%", cpu)).fixed(1),
                            Text::from(vec![
                                span!("MEM: "),
                                span!(format!("{}%", mem), style!(fg = Color::Red)),
                            ])
                            .fixed(1),
                        ]),
                    }
                    .fill(),
                    Bordered {
                        block: Block::new().title("Core 1"),
                        child: VStack::new(vec![
                            Text::raw(format!("CPU: {}%", cpu)).fixed(1),
                            Text::from(vec![
                                span!("MEM: "),
                                span!(format!("{}%", mem), style!(fg = Color::Blue)),
                            ])
                            .fixed(1),
                        ]),
                    }
                    .fill(),
                ]),
                inner,
            );
        });

        cpu += 1;
        mem = mem.saturating_sub(1);

        thread::sleep(Duration::from_millis(500));
    }
}
