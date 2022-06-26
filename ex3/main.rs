fn main()
{
    println!("{}", sqrt(81.0));
}

fn sqrt(x : f64) -> f64
{
    _sqrt(x / 2.0, x)
}

fn _sqrt(p : f64, x : f64) -> f64
{
    let p = if p * p > x {
        p / 2.0
    } else {
        3.0 * p / 2.0
    };
    let mut diff = p * p - x;
    if diff < 0.0 {
        diff = -diff
    }
    if diff < 0.01
    {
        return p;
    }
    else
    {
        return _sqrt(p, x);
    }
}