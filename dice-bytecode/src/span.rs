use std::ops::{Add, Range};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    pub const fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub const fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}

impl Add<&Span> for Span {
    type Output = Span;

    fn add(self, rhs: &Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}

impl Add for &Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}
