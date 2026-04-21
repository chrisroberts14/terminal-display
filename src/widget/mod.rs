use crate::buffer::Buffer;
use crate::geometry::Rect;

pub mod block;
pub mod stack;
pub mod text;

pub use block::Block;
pub use stack::{BoxedWidget, HStack, VStack, boxed};
pub use text::Text;

pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}
