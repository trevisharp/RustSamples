trait Addable<T> {
    fn add(&self, other: &T) -> T;
}

impl Addable<i32> for i32 {
    fn add(&self, other: &i32) -> i32 {
        self + other
    }
}

impl Addable<f32> for f32 {
    fn add(&self, other: &f32) -> f32 {
        self + other
    }
}

fn sum<T : Addable<T>>(list: &[T]) -> &T
    where T: Addable<T>
{
    let mut start = &list[0];
    for i in 1..list.len() {
        start = &start.add(&list[i]);
    }
    &start
}

fn main() {
    let list = [1, 2, 3];
    println!("{}", sum(&list))
}
