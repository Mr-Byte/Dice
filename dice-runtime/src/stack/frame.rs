use std::ops::Range;

#[derive(Copy, Clone, Debug)]
pub struct StackFrame {
    start: usize,
    end: usize,
}

impl StackFrame {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(self) -> usize {
        self.start
    }

    pub fn range(self) -> Range<usize> {
        self.start..self.end
    }

    pub fn prepend(self, count: usize) -> Self {
        Self {
            start: self.start.wrapping_sub(count),
            end: self.end,
        }
    }

    pub fn length(self) -> usize {
        self.end.wrapping_sub(self.start)
    }
}
