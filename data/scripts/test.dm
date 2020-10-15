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

let message = "Hello, world!";

export message;
