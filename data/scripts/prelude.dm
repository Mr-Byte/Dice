export class Range {
    fn new(self, start, end) {
        self.start = start;
        self.end = end;
    }

    fn iter(self) {
        let mut current = self.start;

        #{
            next: || {
                let result = if current < self.end {
                    #{ value: current, is_done: false }
                } else {
                    #{ is_done: true }
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

        #{
            next: || {
                let result = if current <= self.end {
                    #{ value: current, done: false }
                } else {
                    #{ done: true }
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

export class Ok {
    fn new(self, result) {
        self.is_ok = true;
        self.result = result;
    }
}

export class Err {
    fn new(self, result) {
        self.is_ok = false;
        self.result = result;
    }
}