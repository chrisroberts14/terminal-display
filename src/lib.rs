//! Public re-exports

mod buffer;
mod geometry;
mod style;
mod widget;

pub use buffer::{Buffer, Cell};
pub use geometry::Rect;
pub use style::{Color, Span, Style};
pub use widget::{Text, Widget};
