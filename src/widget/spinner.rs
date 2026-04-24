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
        buf.mark_animated();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::style::Style;

    fn rect(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn renders_a_frame_character_from_dots() {
        let mut buf = Buffer::empty(rect(1, 1));
        Spinner::new(SpinnerStyle::Dots, Style::default()).render(rect(1, 1), &mut buf);
        let ch = buf.get_cell(0, 0).unwrap().ch;
        let valid: Vec<char> = SpinnerStyle::Dots
            .frames()
            .iter()
            .flat_map(|s| s.chars())
            .collect();
        assert!(valid.contains(&ch), "unexpected char: {ch}");
    }

    #[test]
    fn all_styles_render_without_panic() {
        for style in [SpinnerStyle::Dots, SpinnerStyle::Line, SpinnerStyle::Arc] {
            let mut buf = Buffer::empty(rect(2, 1));
            Spinner::new(style, Style::default()).render(rect(2, 1), &mut buf);
        }
    }

    #[test]
    fn style_is_applied_to_rendered_cell() {
        let bold = Style {
            bold: true,
            ..Style::default()
        };
        let mut buf = Buffer::empty(rect(1, 1));
        Spinner::new(SpinnerStyle::Line, bold).render(rect(1, 1), &mut buf);
        assert!(buf.get_cell(0, 0).unwrap().style.bold);
    }
}
