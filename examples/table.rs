//! Demonstrates [`Table`] with headers, styled spans, and a selected row.
//!
//! Three columns (`Fixed(10)`, `Fill`, `Fixed(8)`), a bold header row, several
//! data rows including a coloured status span, and row 1 highlighted. Press
//! Ctrl-C to exit.

use std::thread;
use std::time::Duration;
use terminal_display::{Color, Constraint, Row, Table, Terminal, span, style};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let green = style!(fg = Color::Green);
    let red = style!(fg = Color::Red);
    let yellow = style!(fg = Color::Yellow);

    handle.render(move |frame| {
        let rows = vec![
            Row::new(vec![
                vec![span!("alice")],
                vec![span!("Platform Engineer")],
                vec![span!("active", green)],
            ]),
            Row::new(vec![
                vec![span!("bob")],
                vec![span!("Product Manager")],
                vec![span!("active", green)],
            ]),
            Row::new(vec![
                vec![span!("carol")],
                vec![span!("Designer")],
                vec![span!("away", yellow)],
            ]),
            Row::new(vec![
                vec![span!("dave")],
                vec![span!("Backend Engineer")],
                vec![span!("offline", red)],
            ]),
        ];

        frame.render(
            Table::new(
                vec![
                    Constraint::Fixed(10),
                    Constraint::Fill,
                    Constraint::Fixed(8),
                ],
                rows,
            )
            .headers(vec!["Name".into(), "Role".into(), "Status".into()])
            .selected(Some(1)),
            frame.area(),
        );
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
