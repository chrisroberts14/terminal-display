//! All built-in widgets and the [`Widget`] trait.

use crate::buffer::Buffer;
use crate::geometry::Rect;
use crate::layout::Constraint;

pub(crate) mod block;
pub(crate) mod bordered;
pub(crate) mod centered;
pub(crate) mod divider;
pub(crate) mod gauge;
pub(crate) mod progress_bar;
pub(crate) mod spinner;
pub(crate) mod stack;
pub(crate) mod text;

pub use block::Block;
pub use bordered::Bordered;
pub use centered::Centered;
pub use divider::Divider;
pub use gauge::Gauge;
pub use progress_bar::ProgressBar;
pub use spinner::{Spinner, SpinnerStyle};
pub use stack::{BoxedWidget, HStack, VStack, boxed};
pub use text::Text;

/// The core rendering trait. Implement this to create a custom widget.
///
/// `render` writes into `buf` within the bounds of `area`. Out-of-bounds writes are
/// silently ignored by [`Buffer`]. Call [`Buffer::mark_animated`] if the widget
/// produces time-varying output that needs periodic redraws.
pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Returns the widget's intrinsic size as `(width, height)` in terminal cells, if it has one.
    ///
    /// `Centered` uses this to compute a centred sub-rect. Return `None` (the
    /// default) to opt out — the widget will fill whatever area it is given.
    fn natural_size(&self) -> Option<(u16, u16)> {
        None
    }
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
