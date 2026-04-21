use crate::layout::solve;
use crate::{Buffer, Constraint, Rect, Widget};

pub type BoxedWidget = Box<dyn FnOnce(Rect, &mut Buffer)>;

pub fn boxed(w: impl Widget + 'static) -> BoxedWidget {
    Box::new(move |area, buf| w.render(area, buf))
}

pub struct VStack {
    children: Vec<(Constraint, BoxedWidget)>,
}

pub struct HStack {
    children: Vec<(Constraint, BoxedWidget)>,
}

impl VStack {
    pub fn new(children: Vec<(Constraint, BoxedWidget)>) -> Self {
        VStack { children }
    }
}

impl HStack {
    pub fn new(children: Vec<(Constraint, BoxedWidget)>) -> Self {
        HStack { children }
    }
}

impl Widget for VStack {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints: Vec<Constraint> = self.children.iter().map(|(c, _)| *c).collect();
        let heights = solve(&constraints, area.height);
        let mut y = area.y;
        for ((_, child), h) in self.children.into_iter().zip(heights) {
            if h > 0 {
                child(
                    Rect {
                        x: area.x,
                        y,
                        width: area.width,
                        height: h,
                    },
                    buf,
                );
            }
            y += h;
        }
    }
}

impl Widget for HStack {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints: Vec<Constraint> = self.children.iter().map(|(c, _)| *c).collect();
        let widths = solve(&constraints, area.width);
        let mut x = area.x;
        for ((_, child), w) in self.children.into_iter().zip(widths) {
            if w > 0 {
                child(
                    Rect {
                        x,
                        y: area.y,
                        width: w,
                        height: area.height,
                    },
                    buf,
                );
            }
            x += w;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::geometry::Rect;
    use crate::layout::Constraint;
    use crate::widget::Widget;
    use crate::widget::text::Text;

    fn area(w: u16, h: u16) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn vstack_renders_fixed_rows() {
        let mut buf = Buffer::empty(area(10, 2));
        VStack::new(vec![
            (Constraint::Fixed(1), boxed(Text::raw("top"))),
            (Constraint::Fixed(1), boxed(Text::raw("bot"))),
        ])
        .render(area(10, 2), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 't');
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, 'b');
    }

    #[test]
    fn hstack_renders_fixed_cols() {
        let mut buf = Buffer::empty(area(6, 1));
        HStack::new(vec![
            (Constraint::Fixed(3), boxed(Text::raw("abc"))),
            (Constraint::Fixed(3), boxed(Text::raw("def"))),
        ])
        .render(area(6, 1), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 'a');
        assert_eq!(buf.get_cell(3, 0).unwrap().ch, 'd');
    }

    #[test]
    fn vstack_fill_takes_remaining_height() {
        let mut buf = Buffer::empty(area(5, 4));
        VStack::new(vec![
            (Constraint::Fixed(1), boxed(Text::raw("top"))),
            (Constraint::Fill, boxed(Text::raw("fill"))),
        ])
        .render(area(5, 4), &mut buf);
        assert_eq!(buf.get_cell(0, 0).unwrap().ch, 't'); // row 0 = "top"
        assert_eq!(buf.get_cell(0, 1).unwrap().ch, 'f'); // row 1 = "fill"
    }
}
