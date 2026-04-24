use crate::buffer::{Buffer, Cell};
use crate::geometry::Rect;
use crate::style::Color;
use crate::widget::Widget;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::{
    Attribute, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{Write, stdout};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

type RenderFn = Arc<dyn Fn(&mut Frame) + Send + Sync>;

enum Command {
    Render(RenderFn),
    Resize(u16, u16),
    Tick,
    Shutdown,
}

/// A single-frame rendering context passed to the closure in [`TerminalHandle::render`].
///
/// Call [`Frame::render`] one or more times to draw widgets, then return from the closure.
/// The runtime diffs the resulting buffer against the previous frame and writes only
/// changed cells to stdout.
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

    /// Returns the full terminal area available for this frame.
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Renders `widget` into `area`, writing its cells into this frame's buffer.
    ///
    /// Widgets that draw outside `area` are clipped. Call this multiple times to layer
    /// widgets — for example, render a [`Block`](crate::widget::Block) border first, then
    /// render content into its inner area.
    pub fn render(&mut self, widget: impl Widget, area: Rect) {
        widget.render(area, &mut self.buffer);
    }

    pub(crate) fn into_buffer(self) -> Buffer {
        self.buffer
    }
}

/// Owns the terminal for its lifetime and spawns the render and event threads.
///
/// Create with [`Terminal::new`], then immediately call [`Terminal::run`] to transfer
/// ownership to the background threads and receive a [`TerminalHandle`].
pub struct Terminal {
    area: Rect,
}

impl Terminal {
    /// Initialises the terminal: enables raw mode, enters the alternate screen, and hides
    /// the cursor. Returns an error if the terminal cannot be configured.
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

    /// Moves the terminal into a background render thread and returns a [`TerminalHandle`].
    ///
    /// Two threads are spawned:
    /// - **Render thread** — receives render closures and resize commands, diffs each
    ///   frame against the previous one, and writes only changed cells to stdout.
    /// - **Event thread** — blocks on crossterm events and forwards terminal resize
    ///   events to the render thread, which clears the screen and updates the frame area.
    ///   Note: Any other events can be added here
    pub fn run(self) -> TerminalHandle {
        let area = self.area;
        let (tx, rx) = mpsc::channel::<Command>();
        let event_tx = tx.clone();
        let render_tx = tx.clone();

        thread::spawn(move || {
            let mut area = area;
            let mut prev = Buffer::empty(area);
            let mut current_fn: Option<RenderFn> = None;
            let mut tick_running = false;

            'outer: while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::Render(f) => current_fn = Some(f),
                    Command::Resize(w, h) => {
                        area = Rect {
                            x: 0,
                            y: 0,
                            width: w,
                            height: h,
                        };
                        prev = Buffer::empty(area);
                        let _ = execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
                    }
                    Command::Tick => {}
                    Command::Shutdown => break 'outer,
                }
                if let Some(f) = &current_fn {
                    let mut frame = Frame::new(area);
                    f(&mut frame);
                    let curr = frame.into_buffer();
                    if curr.animated && !tick_running {
                        let tick_tx = render_tx.clone();
                        thread::spawn(move || {
                            while tick_tx.send(Command::Tick).is_ok() {
                                thread::sleep(Duration::from_millis(1000 / 24));
                            }
                        });
                        tick_running = true;
                    }
                    let _ = render_diff(&curr, &prev);
                    prev = curr;
                }
            }
            let _ = execute!(stdout(), LeaveAlternateScreen, Show);
            let _ = disable_raw_mode();
        });

        thread::spawn(move || {
            loop {
                match crossterm::event::read() {
                    Ok(Event::Resize(w, h)) => {
                        let _ = event_tx.send(Command::Resize(w, h));
                    }
                    Ok(Event::Key(key))
                        if key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        let _ = event_tx.send(Command::Shutdown);
                        break;
                    }
                    _ => {}
                }
            }
        });

        TerminalHandle { tx }
    }
}

fn render_diff(curr: &Buffer, prev: &Buffer) -> std::io::Result<()> {
    let Some(cells) = curr.diff(prev) else {
        return Ok(());
    };
    let mut out = stdout();
    for (x, y, cell) in cells {
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

/// A cheaply cloneable handle to the background render thread.
///
/// Obtained by calling [`Terminal::run`]. Send render closures with [`TerminalHandle::render`]
/// from any thread. Each closure receives a [`Frame`] and should call [`Frame::render`]
/// to draw widgets before returning.
#[derive(Clone)]
pub struct TerminalHandle {
    tx: mpsc::Sender<Command>,
}

impl TerminalHandle {
    /// Sets the current render function.
    ///
    /// The closure is called immediately to produce the first frame, and again automatically
    /// whenever the terminal is resized, keeping the display up to date without any looping
    /// in the caller. Calling `render` again replaces the closure and triggers a redraw.
    pub fn render(&self, f: impl Fn(&mut Frame) + Send + Sync + 'static) {
        let _ = self.tx.send(Command::Render(Arc::new(f)));
    }

    /// Signals the render thread to restore the terminal and exit.
    ///
    /// After calling this, further `render` calls are silently ignored.
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
        assert_eq!(frame.into_buffer().get_cell(0, 0).unwrap().ch, 'h');
    }

    #[test]
    fn diff_detects_changed_cells() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        };
        let prev = Buffer::empty(area);
        let mut curr = Buffer::empty(area);
        curr.set_str(0, 0, "hi", Style::default());
        let diffs = curr.diff(&prev).unwrap();
        assert_eq!(diffs.len(), 2);
        assert_eq!(diffs[0].2.ch, 'h');
    }
}
