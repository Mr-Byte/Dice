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
                    object { value: current, is_done: false }
                } else {
                    object { is_done: true }
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

op ..(start, end) {
    Range(start, end)
}

op ..=(start, end) {
    RangeInclusive(start, end)
}

op d(value) {
    value
}

op d(lhs, rhs) {
    lhs * rhs
}

export class Test {
    fn new(self, value) {
        self.value = value;
    }

    op +(self, rhs) {
        Test(self.value + rhs.value)
    }
}