//! Convenience macros: [`style!`], [`span!`], [`vstack!`], [`hstack!`].

/// Constructs a [`Style`](crate::Style) using a concise field syntax.
///
/// # Examples
///
/// ```
/// # use terminal_display::{style, Color};
/// let s = style!(bold);
/// let s = style!(fg = Color::Red);
/// let s = style!(fg = Color::Green, bold, italic);
/// ```
#[macro_export]
macro_rules! style {
    ($($key:ident $(= $val:expr)?),* $(,)?) => {{
        let mut s = $crate::Style::default();
        $($crate::_style_field!(s, $key $(= $val)?);)*
        s
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _style_field {
    ($s:ident, fg = $val:expr) => {
        $s.fg = Some($val);
    };
    ($s:ident, bg = $val:expr) => {
        $s.bg = Some($val);
    };
    ($s:ident, bold) => {
        $s.bold = true;
    };
    ($s:ident, underline) => {
        $s.underline = true;
    };
    ($s:ident, italic) => {
        $s.italic = true;
    };
}

/// Constructs a [`Span`](crate::Span) — either unstyled or with a [`Style`](crate::Style).
///
/// # Examples
///
/// ```
/// # use terminal_display::{span, style, Color};
/// let plain = span!("hello");
/// let styled = span!("hello", style!(fg = Color::Red));
/// ```
#[macro_export]
macro_rules! span {
    ($content:expr) => {
        $crate::Span::raw($content)
    };
    ($content:expr, $style:expr) => {
        $crate::Span::styled($content, $style)
    };
}

/// Constructs a [`VStack`](crate::VStack) of plain-text rows, each taking exactly 1 row of height.
///
/// # Examples
///
/// ```
/// # use terminal_display::vstack;
/// let stack = vstack!["line one", "line two", "line three"];
/// ```
#[macro_export]
macro_rules! vstack {
    ($($line:expr),* $(,)?) => {
        $crate::VStack::new(vec![
            $(($crate::Constraint::Fixed(1), $crate::boxed($crate::Text::raw($line)))),*
        ])
    };
}

/// Constructs an [`HStack`](crate::HStack) of plain-text columns, each taking an equal share of the available width.
///
/// # Examples
///
/// ```
/// # use terminal_display::hstack;
/// let stack = hstack!["left", "centre", "right"];
/// ```
#[macro_export]
macro_rules! hstack {
    ($($col:expr),* $(,)?) => {
        $crate::HStack::new(vec![
            $(($crate::Constraint::Fill, $crate::boxed($crate::Text::raw($col)))),*
        ])
    };
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, Style};

    #[test]
    fn style_macro_sets_fg() {
        let s = style!(fg = Color::Red);
        assert_eq!(s.fg, Some(Color::Red));
        assert!(!s.bold);
    }

    #[test]
    fn style_macro_sets_bold() {
        let s = style!(bold);
        assert!(s.bold);
        assert!(s.fg.is_none());
    }

    #[test]
    fn style_macro_sets_multiple() {
        let s = style!(fg = Color::Blue, bold);
        assert_eq!(s.fg, Some(Color::Blue));
        assert!(s.bold);
    }

    #[test]
    fn span_macro_raw() {
        let s = span!("hello");
        assert_eq!(s.content, "hello");
        assert_eq!(s.style, Style::default());
    }

    #[test]
    fn span_macro_styled() {
        let s = span!("hi", style!(fg = Color::Green));
        assert_eq!(s.style.fg, Some(Color::Green));
    }

    #[test]
    fn hstack_macro_creates_fill_columns() {
        use crate::buffer::Buffer;
        use crate::geometry::Rect;
        use crate::widget::Widget;

        let area = Rect {
            x: 0,
            y: 0,
            width: 6,
            height: 1,
        };
        let mut buf = Buffer::empty(area);
        hstack!["ab", "cd"].render(area, &mut buf);
        // each Fill column gets 3 chars: "ab " and "cd "
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'a');
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, 'c');
    }
}
