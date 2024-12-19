struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup(self, other: Point<T, U>) -> Point<T, U> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let x = Point { x: 5, y: 10 };
    let y = Point { x: 11, y: 12 };
    let Point { x: a, y: b } = x;

    println!("{:?}", x.x);
    println!("{:?}", x.x);
    println!("{:?}", x.y);

}
