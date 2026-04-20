use crate::terminal::Terminal;

mod terminal;

fn main() {
    let mut terminal = Terminal::new();

    terminal.set_line(0, format!("stat1: {}", 10));
    terminal.render().expect("TODO: panic message");
}
