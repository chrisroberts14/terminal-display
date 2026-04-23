use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::layout::Constraint;

pub mod block;
pub mod bordered;
pub mod divider;
pub mod spinner;
pub mod stack;
pub mod text;

pub use block::Block;
pub use bordered::Bordered;
pub use divider::Divider;
pub use spinner::{Spinner, SpinnerStyle};
pub use stack::{BoxedWidget, HStack, VStack, boxed};
pub use text::Text;

pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

/// Extension methods for all [`Widget`] types that produce `(Constraint, BoxedWidget)` pairs
/// for use as children in [`VStack`] and [`HStack`].
pub trait WidgetExt: Widget + Sized + 'static {
    /// Use all remaining space, shared equally with other `Fill` siblings.
    fn fill(self) -> (Constraint, BoxedWidget) {
        (Constraint::Fill, boxed(self))
    }

    /// Use exactly `n` rows (in a [`VStack`]) or columns (in an [`HStack`]).
    fn fixed(self, n: u16) -> (Constraint, BoxedWidget) {
        (Constraint::Fixed(n), boxed(self))
    }

    /// Use a fractional share of the total space, e.g. `ratio(1, 3)` = one third.
    fn ratio(self, num: u16, den: u16) -> (Constraint, BoxedWidget) {
        (Constraint::Ratio(num, den), boxed(self))
    }
}

impl<W: Widget + Sized + 'static> WidgetExt for W {}
