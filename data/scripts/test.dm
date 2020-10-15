export class Point {
    fn new(self, x, y, z) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn len_sqr(self) {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

export class Iterator {
    class Result {
        fn new(value, is_done) {
            self.value = value;
            self.is_done = is_done;
        }
    }
}

let message = "Hello, world!";

export message;
