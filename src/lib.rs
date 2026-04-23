//! Public re-exports

pub mod buffer;
pub mod geometry;
pub mod layout;
mod macros;
pub mod style;
pub mod terminal;
pub mod widget;

pub use buffer::{Buffer, Cell};
pub use geometry::Rect;
pub use layout::Constraint;
pub use style::{Color, Span, Style};
pub use terminal::{Frame, Terminal, TerminalHandle};
pub use widget::{
    Block, Bordered, BoxedWidget, HStack, Spinner, SpinnerStyle, Text, VStack, Widget, WidgetExt,
    boxed,
};
