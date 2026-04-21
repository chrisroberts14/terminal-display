use crate::buffer::Buffer;
use crate::geometry::Rect;

pub mod stack;
pub mod text;
pub mod block;

pub use block::Block;
pub use stack::{BoxedWidget, HStack, VStack, boxed};
pub use text::Text;

pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}
