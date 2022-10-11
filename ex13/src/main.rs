
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

impl Addable<f32> for f32 {
    fn add(&self, other: f32) -> f32 {
        self + other
    }
}

fn main() {
    let p = Point(1, 3);
    let q = Point(2, 2);
    let r = p.sum(q);
    
    let p1 = Point(3.0, 1.0);
    let q1 = Point(2.0, 4.0);
    let r1 = p1.sum(q1);
    println!("({}, {})", r.0, r.1);
    println!("({}, {})", r1.0, r1.1)
}