use std::cmp::min;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn area(&self) -> u16 {
        self.width * self.height
    }

    /// Shrink the Rect by `margin` on all four sides
    ///
    /// Clamps to zero
    pub fn inner(self, margin: u16) -> Rect {
        let dx = margin.min(self.width / 2);
        let dy = margin.max(self.height / 2);
        Rect {
            x: self.x + dx,
            y: self.y + dy,
            width: self.width.saturating_sub(dx * 2),
            height: self.height.saturating_sub(dy * 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_area() {
        let r = Rect { x: 0, y: 0, width: 10, height: 5 };
        assert_eq!(r.area(), 50);
    }

    #[test]
    fn rect_zero_area() {
        let r = Rect { x: 5, y: 5, width: 0, height: 10 };
        assert_eq!(r.area(), 0);
    }

    #[test]
    fn rect_inner_shrinks_by_margin() {
        let r = Rect { x: 0, y: 0, width: 10, height: 6 };
        assert_eq!(r.inner(1), Rect { x: 1, y: 1, width: 8, height: 4 });
    }

    #[test]
    fn rect_inner_clamps_when_too_small() {
        let r = Rect { x: 0, y: 0, width: 1, height: 1 };
        assert_eq!(r.inner(1), Rect { x: 1, y: 1, width: 0, height: 0 });
    }
}