use crate::terminal::TerminalHandle;
use crate::widget::progress_bar::ProgressBar;

/// An iterator adapter that renders a [`ProgressBar`] to a [`TerminalHandle`] as it advances.
///
/// Obtain via [`ProgressExt::with_progress`] rather than constructing directly.
pub struct ProgressIter<I: Iterator> {
    inner: I,
    current: usize,
    total: Option<usize>,
    handle: TerminalHandle,
}

impl<I: Iterator> ProgressIter<I> {
    pub fn new(inner: I, handle: TerminalHandle) -> Self {
        let total = match inner.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        };

        handle.render(move |frame| {
            frame.render(ProgressBar::new(0, total), frame.area());
        });

        ProgressIter {
            inner,
            current: 0,
            total,
            handle,
        }
    }
}

impl<I: Iterator> Iterator for ProgressIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();
        if item.is_some() {
            self.current += 1;
            let c = self.current;
            let total = self.total;
            self.handle.render(move |frame| {
                frame.render(ProgressBar::new(c, total), frame.area());
            });
        }
        item
    }
}

impl<I: Iterator> Drop for ProgressIter<I> {
    fn drop(&mut self) {
        let final_current = self.total.unwrap_or(self.current);
        let total = self.total;
        self.handle.render(move |frame| {
            frame.render(ProgressBar::new(final_current, total), frame.area());
        });
    }
}

/// Extension trait that adds [`with_progress`](ProgressExt::with_progress) to any iterator.
pub trait ProgressExt: Iterator + Sized {
    /// Wraps the iterator in a [`ProgressIter`] that renders a progress bar to `handle`
    /// as the loop advances. The bar completes automatically when the iterator is dropped,
    /// even on an early `break`.
    fn with_progress(self, handle: TerminalHandle) -> ProgressIter<Self>;
}

impl<I: Iterator + Sized> ProgressExt for I {
    fn with_progress(self, handle: TerminalHandle) -> ProgressIter<Self> {
        ProgressIter::new(self, handle)
    }
}
