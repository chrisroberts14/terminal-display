use crate::widget::block::Block;
use crate::{Buffer, Rect, Widget};

/// Wraps any widget in a [`Block`] border, rendering the block first then the
/// child inside its inner area.
pub struct Bordered<W: Widget> {
    pub block: Block,
    pub child: W,
}

impl<W: Widget + 'static> Widget for Bordered<W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.block.inner(area);
        self.block.render(area, buf);
        self.child.render(inner, buf);
    }
}
