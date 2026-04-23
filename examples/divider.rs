//! Demonstrates the [`Divider`] widget used as a horizontal rule in a [`VStack`]
//! and as a vertical rule in an [`HStack`].

use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Color, Divider, HStack, Terminal, Text, VStack, WidgetExt, style,
};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        let area = frame.area();
        frame.render(
            Bordered {
                block: Block::new().title("Divider demo"),
                child: VStack::new(vec![
                    // Top half: two text panels separated by a vertical divider
                    HStack::new(vec![
                        Text::raw("Left panel").fill(),
                        Divider::styled(style!(fg = Color::BrightBlack)).fixed(1),
                        Text::raw("Right panel").fill(),
                    ])
                    .fill(),
                    // Horizontal rule between the two halves
                    Divider::new().fixed(1),
                    // Bottom half: three text panels separated by vertical dividers
                    HStack::new(vec![
                        Text::raw("Column A").fill(),
                        Divider::styled(style!(fg = Color::BrightBlack)).fixed(1),
                        Text::raw("Column B").fill(),
                        Divider::styled(style!(fg = Color::BrightBlack)).fixed(1),
                        Text::raw("Column C").fill(),
                    ])
                    .fill(),
                ]),
            },
            area,
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
