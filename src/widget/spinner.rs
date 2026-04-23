use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::widget::Widget;
use crate::widget::text::Text;
use std::time::SystemTime;

/// Built-in frame sequences for a [`Spinner`].
pub enum SpinnerStyle {
    Dots,
    Line,
    Arc,
}

impl SpinnerStyle {
    fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
            SpinnerStyle::Line => &["-", "\\", "|", "/"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
        }
    }

    fn frame_ms(&self) -> u128 {
        match self {
            SpinnerStyle::Dots => 80,
            SpinnerStyle::Line => 120,
            SpinnerStyle::Arc => 100,
        }
    }
}

/// A stateless animated spinner. The current frame is derived from the wall clock,
/// so any number of spinners can be created and discarded freely each render.
///
/// Animation requires [`TerminalHandle::animate`] to be called once to start the
/// background tick thread.
pub struct Spinner {
    style: SpinnerStyle,
}

impl Spinner {
    pub fn new(style: SpinnerStyle) -> Self {
        Spinner { style }
    }
}

impl Widget for Spinner {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let frames = self.style.frames();
        let ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let idx = (ms / self.style.frame_ms()) as usize % frames.len();
        Text::raw(frames[idx]).render(area, buf);
    }
}