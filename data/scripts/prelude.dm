export class Range {
    fn new(self, start, end) {
        self.current = start;
        self.end = end;
    }

    fn next(self) {
        let result = if self.current < self.end {
            object { value: self.current, done: false }
        } else {
            object { done: true }
        };

        self.current += 1;

        result
    }
}

export class RangeInclusive {
    fn new(self, start, end) {
        self.current = start;
        self.end = end;
    }

    fn next(self) {
        let result = if self.current <= self.end {
            object { value: self.current, done: false }
        } else {
            object { done: true }
        };

        self.current += 1;

        result
    }
}

op #range_exclusive(start, end) {
    Range(start, end)
}

op #range_inclusive(start, end) {
    RangeInclusive(start, end)
}