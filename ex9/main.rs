#[derive(Debug)]
enum Option<T>
{
    None,
    Some(T),
}

impl Option<i32>
{
    fn plus_one(&self) -> Option<i32>
    {
        match self {
            Option::None => Option::None,
            Option::Some(i) => Option::Some(i + 1),
        }
    }
}

fn main()
{
    let five = Option::<i32>::Some(5);
    let six = five.plus_one();
    let none = Option::<i32>::None;
    println!("{:?}", six);
    println!("{:?}", none);
}