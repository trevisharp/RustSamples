fn main()
{
    for x in 0..9 {
        println!("{}", x)
    }
    println!("{}", sqrt(81.0));
}

fn sqrt(x : f64) -> f64
{
    let mut p = x / 2.0;
    loop
    {
        let newp = (p * p + x) / (2.0 * p);
        if newp == p {
            return p;
        }
        p = newp;
    }
}