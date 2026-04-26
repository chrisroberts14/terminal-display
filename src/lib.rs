//! A minimal terminal UI library built on [crossterm](https://docs.rs/crossterm).
//!
//! # Quick start
//!
//! ```no_run
//! use terminal_display::{Terminal, Spinner, SpinnerStyle, style};
//!
//! let terminal = Terminal::new().unwrap();
//! let handle = terminal.run();
//! handle.render(|frame| {
//!     frame.render(Spinner::new(SpinnerStyle::Dots, style!(bold)), frame.area());
//! });
//! ```
//!
//! # Architecture
//!
//! - [`Terminal`] initialises the terminal and spawns background threads via [`Terminal::run`].
//! - [`TerminalHandle`] is a cheap, cloneable handle for sending render closures to those threads.
//! - Widgets implement [`Widget`] and write into a [`Buffer`] during [`Frame::render`].
//! - Layout is handled by [`VStack`] / [`HStack`] with [`Constraint`]-based sizing.
//!
//! # Public re-exports

pub(crate) mod buffer;
pub(crate) mod geometry;
pub(crate) mod layout;
mod macros;
pub mod progress;
pub(crate) mod style;
pub(crate) mod terminal;
pub(crate) mod widget;

pub use buffer::{Buffer, Cell};
pub use geometry::Rect;
pub use layout::Constraint;
pub use progress::ProgressExt;
pub use style::{Color, Span, Style};
pub use terminal::{Frame, Terminal, TerminalHandle};
pub use widget::{
    Block, Bordered, BoxedWidget, Centered, Divider, Gauge, HStack, ProgressBar, Spinner,
    SpinnerStyle, Text, VStack, Widget, WidgetExt, boxed,
};
