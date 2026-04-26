//! Demonstrates the [`Gauge`] widget with an animated fill cycling 0 % → 100 %.

use std::thread;
use std::time::Duration;
use terminal_display::{Color, Gauge, Rect, Terminal, style};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let mut step = 0u32;
    let fill = style!(fg = Color::Green);

    loop {
        let value = (step % 101) as f64 / 100.0;
        handle.render(move |frame| {
            let area = frame.area();
            let w = 30u16.min(area.width);
            let h = 15u16.min(area.height);
            let small = Rect {
                x: area.x + (area.width - w) / 2,
                y: area.y + (area.height - h) / 2,
                width: w,
                height: h,
            };
            frame.render(Gauge::new(value).fill_style(fill), small);
        });
        step += 1;
        thread::sleep(Duration::from_millis(50));
    }
}
