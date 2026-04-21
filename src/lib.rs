//! Public re-exports

mod buffer;
mod geometry;
mod layout;
mod style;
mod widget;

pub use buffer::{Buffer, Cell};
pub use geometry::Rect;
pub use layout::Constraint;
pub use style::{Color, Span, Style};
pub use widget::{BoxedWidget, HStack, Text, VStack, Widget, boxed};
