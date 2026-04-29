use std::thread;
use std::time::Duration;
use terminal_display::{
    Block, Bordered, Buffer, Centered, Color, Constraint, Fill, HStack, Rect, Terminal, WidgetExt,
    style,
};

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

fn rainbow(n: usize) -> Vec<(u8, u8, u8)> {
    (0..n)
        .map(|i| {
            let hue = (i as f64 / n as f64) * 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        })
        .collect()
}

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    handle.render(|frame| {
        // Colours
        let rgb_values = rainbow(150);
        let area = frame.area();
        frame.render(
            Bordered::new(
                Block::new().title("Rainbow"),
                HStack::new(
                    rgb_values
                        .iter()
                        .map(|&(r, g, b)| Fill::new(style!(bg = Color::Rgb(r, g, b))).fixed(1))
                        .collect(),
                ),
            ),
            area,
        )
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
