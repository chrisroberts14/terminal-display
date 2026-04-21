use crossterm::style::Color as CtColor;

/// Colors
///
/// Here we define multiple defaults, or you can use your own RGB value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

pub fn to_ct_color(c: Color) -> CtColor {
    match c {
        Color::Reset        => CtColor::Reset,
        Color::Black        => CtColor::Black,
        Color::Red          => CtColor::DarkRed,
        Color::Green        => CtColor::DarkGreen,
        Color::Yellow       => CtColor::DarkYellow,
        Color::Blue         => CtColor::DarkBlue,
        Color::Magenta      => CtColor::DarkMagenta,
        Color::Cyan         => CtColor::DarkCyan,
        Color::White        => CtColor::Grey,
        Color::BrightBlack  => CtColor::DarkGrey,
        Color::BrightRed    => CtColor::Red,
        Color::BrightGreen  => CtColor::Green,
        Color::BrightYellow => CtColor::Yellow,
        Color::BrightBlue   => CtColor::Blue,
        Color::BrightMagenta=> CtColor::Magenta,
        Color::BrightCyan   => CtColor::Cyan,
        Color::BrightWhite  => CtColor::White,
        Color::Rgb(r, g, b) => CtColor::Rgb { r, g, b },
        Color::Indexed(i)   => CtColor::AnsiValue(i),
    }
}

/// Text appearance applied to a cell or [`Span`].
///
/// All fields are additive — unset fields inherit from whatever was beneath them.
/// Use [`Style::patch`] to layer one style on top of another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    /// Foreground (text) colour. `None` means inherit.
    pub fg: Option<Color>,
    /// Background colour. `None` means inherit.
    pub bg: Option<Color>,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

impl Style {
    /// Patch `self` with a new style.
    ///
    /// Only non-None/true fields in `patch` override the values in `self`
    pub fn patch(self, patch: Style) -> Style {
        Style {
            fg: patch.fg.or(self.fg),
            bg: patch.bg.or(self.bg),
            bold: self.bold || patch.bold,
            underline: self.underline || patch.underline,
            italic: self.italic || patch.italic,
        }
    }
}

/// A string fragment paired with a [`Style`].
///
/// A line of rich text is represented as `Vec<Span>`, where each span can carry
/// independent colors and attributes. Adjacent spans with the same style are
/// functionally identical to a single span — splitting is purely for convenience.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub content: String,
    pub style: Style,
}

impl Span {
    /// Creates a `Span` with default (unstyled) appearance.
    pub fn raw(content: impl Into<String>) -> Self {
        Span {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Creates a `Span` with the given [`Style`].
    pub fn styled(content: impl Into<String>, style: Style) -> Self {
        Span {
            content: content.into(),
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_style_has_no_colour() {
        let s = Style::default();
        assert!(s.fg.is_none());
        assert!(s.bg.is_none());
        assert!(!s.bold);
    }

    #[test]
    fn style_patch_overrides_only_set_fields() {
        let base = Style {
            fg: Some(Color::Red),
            ..Style::default()
        };
        let patch = Style {
            bold: true,
            ..Style::default()
        };
        let merged = base.patch(patch);
        assert_eq!(merged.fg, Some(Color::Red));
        assert!(merged.bold);
    }

    #[test]
    fn span_raw_has_default_style() {
        let s = Span::raw("hello");
        assert_eq!(s.content, "hello");
        assert_eq!(s.style, Style::default());
    }

    #[test]
    fn span_styled_sets_style() {
        let style = Style {
            bold: true,
            ..Style::default()
        };
        let s = Span::styled("hi", style);
        assert!(s.style.bold);
    }
}
