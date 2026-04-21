use crate::buffer::{Buffer, Cell};
use crate::geometry::Rect;
use crate::style::Color;
use crate::widget::Widget;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::{
    Attribute, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{Write, stdout};
use std::sync::mpsc;
use std::thread;
use crossterm::event::Event;

enum Command {
    Render(Box<dyn FnOnce(&mut Frame) + Send>),
    Resize(u16, u16),
    Shutdown,
}

pub struct Frame {
    area: Rect,
    buffer: Buffer,
}

impl Frame {
    pub(crate) fn new(area: Rect) -> Self {
        Frame {
            area,
            buffer: Buffer::empty(area),
        }
    }

    pub fn area(&self) -> Rect {
        self.area
    }

    pub fn render(&mut self, widget: impl Widget, area: Rect) {
        widget.render(area, &mut self.buffer);
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub(crate) fn into_buffer(self) -> Buffer {
        self.buffer
    }
}

pub struct Terminal {
    area: Rect,
}

impl Terminal {
    pub fn new() -> std::io::Result<Terminal> {
        let (width, height) = crossterm::terminal::size()?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        Ok(Terminal {
            area: Rect {
                x: 0,
                y: 0,
                width,
                height,
            },
        })
    }

    pub fn run(self) -> TerminalHandle {
        let area = self.area;
        let (tx, rx) = mpsc::channel::<Command>();
        let event_tx = tx.clone();

        thread::spawn(move || {
            let mut area = area;
            let mut prev = Buffer::empty(area);

            while let Ok(command) = rx.recv() {
                match command {
                    Command::Render(func) => {
                        let mut frame = Frame::new(area);
                        func(&mut frame);
                        let curr = frame.into_buffer();
                        render_diff(&curr, &prev).unwrap();
                        prev = curr;
                    }
                    Command::Resize(w, h) => {
                        area = Rect { x: 0, y: 0, width: w, height: h };
                        prev = Buffer::empty(area);
                        let _ = execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
                    }
                    Command::Shutdown => break,
                }
            }
            let _ = execute!(stdout(), LeaveAlternateScreen, Show);
            let _ = disable_raw_mode();
        });

        thread::spawn(move || {
            loop {
                if let Ok(Event::Resize(w, h)) = crossterm::event::read() {
                    let _ = event_tx.send(Command::Resize(w, h));
                }
            }
        });

        TerminalHandle { tx }
    }
}

fn render_diff(curr: &Buffer, prev: &Buffer) -> std::io::Result<()> {
    let mut out = stdout();
    for (x, y, cell) in curr.diff(prev).unwrap() {
        execute!(out, MoveTo(x, y))?;
        apply_style(&cell, &mut out)?;
        execute!(out, Print(cell.ch))?;
    }
    execute!(out, ResetColor)?;
    out.flush()?;
    Ok(())
}

fn apply_style(cell: &Cell, out: &mut impl Write) -> std::io::Result<()> {
    execute!(out, ResetColor)?;
    if let Some(fg) = cell.style.fg {
        execute!(out, SetForegroundColor(to_ct_color(fg)))?;
    }
    if let Some(bg) = cell.style.bg {
        execute!(out, SetBackgroundColor(to_ct_color(bg)))?;
    }
    if cell.style.bold {
        execute!(out, SetAttribute(Attribute::Bold))?;
    }
    if cell.style.underline {
        execute!(out, SetAttribute(Attribute::Underlined))?;
    }
    if cell.style.italic {
        execute!(out, SetAttribute(Attribute::Italic))?;
    }
    Ok(())
}

fn to_ct_color(c: Color) -> crossterm::style::Color {
    use crossterm::style::Color as C;
    match c {
        Color::Reset => C::Reset,
        Color::Black => C::Black,
        Color::Red => C::DarkRed,
        Color::Green => C::DarkGreen,
        Color::Yellow => C::DarkYellow,
        Color::Blue => C::DarkBlue,
        Color::Magenta => C::DarkMagenta,
        Color::Cyan => C::DarkCyan,
        Color::White => C::Grey,
        Color::BrightBlack => C::DarkGrey,
        Color::BrightRed => C::Red,
        Color::BrightGreen => C::Green,
        Color::BrightYellow => C::Yellow,
        Color::BrightBlue => C::Blue,
        Color::BrightMagenta => C::Magenta,
        Color::BrightCyan => C::Cyan,
        Color::BrightWhite => C::White,
        Color::Rgb(r, g, b) => C::Rgb { r, g, b },
        Color::Indexed(i) => C::AnsiValue(i),
    }
}

pub struct TerminalHandle {
    tx: mpsc::Sender<Command>,
}

impl TerminalHandle {
    pub fn render(&self, f: impl FnOnce(&mut Frame) + Send + 'static) {
        let _ = self.tx.send(Command::Render(Box::new(f)));
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(Command::Shutdown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;
    use crate::style::Style;
    use crate::widget::text::Text;

    #[test]
    fn frame_area_matches_constructed_size() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 80,
            height: 24,
        };
        let frame = Frame::new(area);
        assert_eq!(frame.area(), area);
    }

    #[test]
    fn frame_render_writes_widget_into_buffer() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 1,
        };
        let mut frame = Frame::new(area);
        frame.render(Text::raw("hello"), area);
        assert_eq!(frame.buffer().get_cell(0, 0).unwrap().ch, 'h');
    }

    #[test]
    fn diff_detects_changed_cells() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        };
        let prev = crate::buffer::Buffer::empty(area);
        let mut curr = crate::buffer::Buffer::empty(area);
        curr.set_str(0, 0, "hi", Style::default());
        let diffs = curr.diff(&prev).unwrap();
        assert_eq!(diffs.len(), 2);
        assert_eq!(diffs[0].2.ch, 'h');
    }
}
