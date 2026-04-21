use crate::buffer::Buffer;
use crate::geometry::Rect;

pub mod text;

pub use text::Text;

pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}
