use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::widget::Widget;
use crate::widget::text::Text;
use crate::{Span, Style};
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
    spinner_style: SpinnerStyle,
    style: Style,
}

impl Spinner {
    pub fn new(spinner_style: SpinnerStyle, style: Style) -> Self {
        Spinner {
            spinner_style,
            style,
        }
    }
}

impl Widget for Spinner {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let frames = self.spinner_style.frames();
        let ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let idx = (ms / self.spinner_style.frame_ms()) as usize % frames.len();
        let span = Span::styled(frames[idx], self.style);
        Text::from(vec![span]).render(area, buf);
    }
}
