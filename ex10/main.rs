use std::collections::HashMap;

fn main()
{
    let mut u = Vec::new();

    u.push(1);
    u.push(2);
    u.push(3);
    u.push(4);

    for i in &mut u
    {
        println!("{}", i);
    }

    u[0] = 5;

    let first = &u[0];
    // u.push(5); //This line make a error in next line
    println!("{}", first);

    let v = vec![6, 7, 8];
    for i in &v
    {
        println!("{}", i);
    }
    
    for i in v //borrow occours here
    {
        println!("{}", i + 3);
    }
    
    // for i in v//error here
    // {
    //     println!("{}", i);
    // }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}