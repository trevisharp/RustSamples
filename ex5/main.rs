fn main() {
    let s = String::from("rust is cool");

    print_len(s);

    // let s1 = s; Ownership error

    let s = String::from("rust is cool");

    print_len2(s);
}

fn print_len(s: String)
{
    let (s, len) = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();

    (s, length)
}

fn print_len2(s: String)
{
    let len = calculate_length2(&s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length2(s: &String) -> usize
{
    s.len()
}