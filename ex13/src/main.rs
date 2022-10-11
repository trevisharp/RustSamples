
struct Point<T : Addable<T>>(T, T);

trait Addable<T> {
    fn add(&self, other: T) -> T;
}

impl<T : Addable<T>> Point<T> {
    fn sum(&self, other: Point<T>) -> Point<T> {
        Point(self.0.add(other.0), self.1.add(other.1))
    }
}

impl Addable<i32> for i32 {
    fn add(&self, other: i32) -> i32 {
        self + other
    }
}

fn main() {
    let p = Point(1, 3);
    let q = Point(2, 2);
    let r = p.sum(q);
    println!("({}, {})", r.0, r.1)
}