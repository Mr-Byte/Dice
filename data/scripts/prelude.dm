export class Range {
    fn new(self, start, end) {
        self.start = start
        self.end = end
    }

    fn iter(self) {
        let mut current = self.start

        #{
            next: || {
                let result = if current < self.end {
                    #{ value: current, is_done: false }
                } else {
                    #{ is_done: true }
                }

                current += 1

                result
            }
        }
    }
}

export class RangeInclusive {
    fn new(self, start, end) {
        self.start = start
        self.end = end
    }

    fn iter(self) {
        let mut current = self.start

        #{
            next: || {
                let result = if current <= self.end {
                    #{ value: current, done: false }
                } else {
                    #{ done: true }
                }

                current += 1

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

export class Result {
    fn new(self, is_ok: Bool, result: Any?) {
        self.is_ok = is_ok
        self.result = result
    }

    fn map(self, map_fn: Function) -> Result {
        if self.is_ok {
            Ok(map_fn(self.result))
        } else {
            Err(self.result)
        }
    }
}

export class Ok : Result {
    fn new(self, result) {
        super(true, result)
    }
}

export class Err : Result {
    fn new(self, result) {
        super(false, result)
    }
}

export fn breaks() {
    panic("OH NOES D:")
}

export fn test() {
    breaks()
}
