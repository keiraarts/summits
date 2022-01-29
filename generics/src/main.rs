struct Point<X1, Y2> {
    x: X1,
    y: Y2,
}

impl<X1, Y1> Point<X1, Y1> {
    fn swap<X2, Y2>(self, options: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: options.y,
        }
    }
}

fn main() {
    // https://users.rust-lang.org/t/confusion-struct-impl-self-trait/3941/4
    println!("Hello, world!");
    let p1 = Point { x: 20, y: 50 };
    let p2 = Point { x: 5, y: 2 };
    p1.swap(p2);
}
