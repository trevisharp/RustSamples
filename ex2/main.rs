fn main()
{
    let x = {
        let x = seven();
        x + 1
    };
    print(x);
    println!("{}", sqrt(81.0))
}

fn seven() -> i32
{
    7
}

fn print(x: i32)
{
    println!("{}", x);
}

fn sqrt(x : f64) -> f64
{
    let mut p = x / 50.0;
    p = (p * p + x) / (2.0 * p);
    p = (p * p + x) / (2.0 * p);
    p = (p * p + x) / (2.0 * p);
    p = (p * p + x) / (2.0 * p);
    p = (p * p + x) / (2.0 * p);
    p = (p * p + x) / (2.0 * p);
    p    
}