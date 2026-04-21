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

#[macro_export]
macro_rules! span {
    ($content:expr) => {
        $crate::Span::raw($content)
    };
    ($content:expr, $style:expr) => {
        $crate::Span::styled($content, $style)
    };
}

/// Builds a `VStack` of plain-text rows, each taking exactly 1 row of height.
#[macro_export]
macro_rules! vstack {
    ($($line:expr),* $(,)?) => {
        $crate::VStack::new(vec![
            $(($crate::Constraint::Fixed(1), $crate::boxed($crate::Text::raw($line)))),*
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
}
