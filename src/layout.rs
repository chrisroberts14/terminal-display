//! Constraint-based layout solver used by [`VStack`](crate::widget::VStack) and [`HStack`](crate::widget::HStack).

/// Describes how a child in a [`VStack`](crate::widget::VStack) or
/// [`HStack`](crate::widget::HStack) should be sized.
#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    /// Exactly `n` rows or columns.
    Fixed(u16),
    /// Take an equal share of whatever space remains after `Fixed` and `Ratio` children.
    Fill,
    /// A fractional share of the total — e.g. `Ratio(1, 3)` is one third.
    Ratio(u16, u16),
}

/// Allocates `total` units across `constraints`. Returns one length per constraint.
pub(crate) fn solve(constraints: &[Constraint], total: u16) -> Vec<u16> {
    let mut sizes = vec![0u16; constraints.len()];
    let mut used: u16 = 0;
    let mut fill_count: u16 = 0;

    for (i, c) in constraints.iter().enumerate() {
        match c {
            Constraint::Fixed(n) => {
                let alloc = (*n).min(total.saturating_sub(used));
                sizes[i] = alloc;
                used = used.saturating_add(alloc);
            }
            Constraint::Ratio(num, den) => {
                let alloc = if *den == 0 {
                    0
                } else {
                    (total * num / den).min(total.saturating_sub(used))
                };
                sizes[i] = alloc;
                used = used.saturating_add(alloc);
            }
            Constraint::Fill => fill_count += 1,
        }
    }

    if fill_count > 0 {
        let remaining = total.saturating_sub(used);
        let per_fill = remaining / fill_count;
        let mut extra = remaining % fill_count;
        for (i, c) in constraints.iter().enumerate() {
            if matches!(c, Constraint::Fill) {
                sizes[i] = per_fill
                    + if extra > 0 {
                        extra -= 1;
                        1
                    } else {
                        0
                    };
            }
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_constraints_exact() {
        let sizes = solve(&[Constraint::Fixed(3), Constraint::Fixed(5)], 20);
        assert_eq!(sizes, vec![3, 5]);
    }

    #[test]
    fn fill_splits_remaining_equally() {
        // 20 - 4 fixed = 16 remaining, split between 2 Fill → 8 each
        let sizes = solve(
            &[Constraint::Fixed(4), Constraint::Fill, Constraint::Fill],
            20,
        );
        assert_eq!(sizes, vec![4, 8, 8]);
    }

    #[test]
    fn ratio_allocates_fractional_share() {
        // Ratio(1,3) and Ratio(2,3) of 30 → 10 and 20
        let sizes = solve(&[Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)], 30);
        assert_eq!(sizes, vec![10, 20]);
    }

    #[test]
    fn fill_gets_zero_when_no_space_remains() {
        // Fixed(20) on a total of 10 — clamp, Fill gets nothing
        let sizes = solve(&[Constraint::Fixed(20), Constraint::Fill], 10);
        assert_eq!(sizes, vec![10, 0]);
    }

    #[test]
    fn mixed_fixed_and_fill() {
        // 5 + Fill + 3 in 20 → Fill gets 20 - 5 - 3 = 12
        let sizes = solve(
            &[Constraint::Fixed(5), Constraint::Fill, Constraint::Fixed(3)],
            20,
        );
        assert_eq!(sizes, vec![5, 12, 3]);
    }
}
