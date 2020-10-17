export class Range {
    fn new(self, start, end) {
        self.start = start;
        self.end = end;
    }

    fn iter(self) {
        let mut current = self.start;
        object {
            next: || {
                let result = if current < self.end {
                    object { value: current, done: false }
                } else {
                    object { done: true }
                };

                current += 1;

                result
            }
        }
    }
}

export class RangeInclusive {
    fn new(self, start, end) {
        self.start = start;
        self.end = end;
    }

    fn iter(self) {
        let mut current = self.start;
        object {
            next: || {
                let result = if current <= self.end {
                    object { value: current, done: false }
                } else {
                    object { done: true }
                };

                current += 1;

                result
            }
        }
    }
}

op #range_exclusive(start, end) {
    Range(start, end)
}

op #range_inclusive(start, end) {
    RangeInclusive(start, end)
}